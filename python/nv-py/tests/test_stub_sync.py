"""Deterministic stub/binding sync tests for `nv_rs` (no GPU or compiled extension required)."""

from __future__ import annotations

import ast
import re
import unittest
from pathlib import Path

try:
    import tomllib  # Python 3.11+
except ModuleNotFoundError:  # pragma: no cover - Python 3.10 fallback
    tomllib = None  # type: ignore[assignment]

NV_PY_ROOT = Path(__file__).resolve().parent.parent
SRC_DIR = NV_PY_ROOT / "src"
LIB_RS = SRC_DIR / "lib.rs"
PKG_DIR = NV_PY_ROOT / "python" / "nv_rs"
INIT_PY = PKG_DIR / "__init__.py"
INIT_PYI = PKG_DIR / "__init__.pyi"
PY_TYPED = PKG_DIR / "py.typed"
PYPROJECT_TOML = NV_PY_ROOT / "pyproject.toml"


def _extract_brace_block(text: str, open_brace_idx: int) -> str:
    """Return the substring from `{` at `open_brace_idx` through its matching `}`."""
    depth = 0
    for idx in range(open_brace_idx, len(text)):
        ch = text[idx]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return text[open_brace_idx : idx + 1]
    raise AssertionError("Unbalanced braces while parsing Rust source")


def _parse_rust_registered_submodules() -> tuple[set[str], set[str]]:
    """Return `(registered_submodules, top_level_attrs)` parsed from `src/lib.rs` and its modules."""
    lib_text = LIB_RS.read_text(encoding="utf-8")

    # Extract `<mod>::register(py, m)?;` inside `_nv_rs`.
    reg_mods = re.findall(r"\b([a-zA-Z_][a-zA-Z0-9_]*)::register\s*\(", lib_text)
    if not reg_mods:
        raise AssertionError(f"No `<mod>::register(...)` calls found in {LIB_RS}")

    top_attrs = set(re.findall(r'\bm\.add\(\s*"([a-zA-Z_][a-zA-Z0-9_]*)"\s*,', lib_text))

    submodules: set[str] = set()
    for mod_name in reg_mods:
        mod_path = SRC_DIR / f"{mod_name}.rs"
        if not mod_path.is_file():
            raise AssertionError(f"Registered module `{mod_name}` missing source file: {mod_path}")
        mod_text = mod_path.read_text(encoding="utf-8")

        macro_found = re.findall(
            r'py_register_submodule!\s*\([^,]+,[^,]+,\s*"([a-zA-Z_][a-zA-Z0-9_]*)"\s*,',
            mod_text,
        )
        manual_found = re.findall(
            r'PyModule::new\s*\(\s*py\s*,\s*"([a-zA-Z_][a-zA-Z0-9_]*)"\s*\)',
            mod_text,
        )
        if manual_found and "add_submodule" not in mod_text:
            raise AssertionError(f"No `add_submodule` call found in {mod_path}")

        found = macro_found + manual_found
        if not found:
            raise AssertionError(
                f"No `py_register_submodule!` or `PyModule::new(py, \"...\")` found in {mod_path}"
            )
        submodules.update(found)

    return submodules, top_attrs


def _parse_rust_submodule_items() -> tuple[
    dict[str, set[str]],
    dict[str, set[str]],
    dict[str, set[str]],
    set[str],
    set[str],
]:
    """Parse Rust `src/*.rs` for submodule classes/functions and `#[pymethods]` methods.

    Returns `(submod_classes, submod_funcs, class_methods, all_pyclasses, all_pyfunctions)`.
    """
    lib_text = LIB_RS.read_text(encoding="utf-8")
    reg_mods = re.findall(r"\b([a-zA-Z_][a-zA-Z0-9_]*)::register\s*\(", lib_text)

    submod_classes: dict[str, set[str]] = {}
    submod_funcs: dict[str, set[str]] = {}
    class_methods: dict[str, set[str]] = {}
    all_pyclasses: set[str] = set()
    all_pyfunctions: set[str] = set()

    for mod_name in reg_mods:
        mod_path = SRC_DIR / f"{mod_name}.rs"
        mod_text = mod_path.read_text(encoding="utf-8")

        # Map Rust struct -> Python class name for `#[pyclass]` in this file.
        struct_to_py: dict[str, str] = {}
        for m in re.finditer(
            r"#\[pyclass(?:\(([^\)]*)\))?\]\s*(?:#\[[^\]]+\]\s*)*(?:pub(?:\([^\)]+\))?\s+)?struct\s+([A-Za-z_][A-Za-z0-9_]*)",
            mod_text,
        ):
            attrs = m.group(1) or ""
            rust_name = m.group(2)
            name_m = re.search(r'\bname\s*=\s*"([^"]+)"', attrs)
            py_name = name_m.group(1) if name_m else rust_name
            struct_to_py[rust_name] = py_name
            all_pyclasses.add(py_name)

        # Map Rust fn -> Python function name for `#[pyfunction]` in this file.
        fn_to_py: dict[str, str] = {}
        for m in re.finditer(
            r"((?:\s*#\[[^\]]+\]\s*)*)\s*(?:pub(?:\([^\)]+\))?\s+)?(?:unsafe\s+)?fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*(?:<[^>]*>)?\s*\(",
            mod_text,
        ):
            attrs = m.group(1)
            if "#[pyfunction" not in attrs:
                continue
            rust_fn = m.group(2)
            name_m = re.search(r'\bname\s*=\s*"([^"]+)"', attrs)
            py_fn = name_m.group(1) if name_m else rust_fn
            fn_to_py[rust_fn] = py_fn
            all_pyfunctions.add(py_fn)

        # Extract `#[pymethods] impl <Struct> { ... }` methods.
        for m in re.finditer(r"#\[pymethods\]\s*impl\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{", mod_text):
            rust_struct = m.group(1)
            py_cls = struct_to_py.get(rust_struct, rust_struct)
            body = _extract_brace_block(mod_text, m.end() - 1)
            methods = class_methods.setdefault(py_cls, set())
            for fm in re.finditer(
                r"((?:\s*#\[[^\]]+\]\s*)*)\s*(?:pub(?:\([^\)]+\))?\s+)?(?:unsafe\s+)?fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*(?:<[^>]*>)?\s*\(",
                body,
            ):
                fn_pos = fm.start(2)
                line_start = body.rfind("\n", 0, fn_pos) + 1
                prefix = body[line_start:fn_pos]
                if len(prefix) - len(prefix.lstrip(" ")) != 4:
                    continue
                attrs = fm.group(1)
                fn_name = fm.group(2)
                if "#[new]" in attrs:
                    methods.add("__init__")
                else:
                    name_m = re.search(r'\bname\s*=\s*"([^"]+)"', attrs)
                    methods.add(name_m.group(1) if name_m else fn_name)

        # Extract items registered into each `py_register_submodule!` block.
        for m in re.finditer(
            r'py_register_submodule!\s*\([^,]+,[^,]+,\s*"([a-zA-Z_][a-zA-Z0-9_]*)"\s*,\s*\|([a-zA-Z_][a-zA-Z0-9_]*)\|\s*\{',
            mod_text,
        ):
            sub_name = m.group(1)
            var_name = m.group(2)
            body = _extract_brace_block(mod_text, m.end() - 1)

            classes = submod_classes.setdefault(sub_name, set())
            funcs = submod_funcs.setdefault(sub_name, set())

            for cm in re.finditer(rf"\b{re.escape(var_name)}\.add_class::<([A-Za-z_][A-Za-z0-9_]*)>\(\)", body):
                rust_struct = cm.group(1)
                classes.add(struct_to_py.get(rust_struct, rust_struct))

            for fm in re.finditer(
                rf"\b{re.escape(var_name)}\.add_function\(wrap_pyfunction!\(([A-Za-z_][A-Za-z0-9_]*)",
                body,
            ):
                rust_fn = fm.group(1)
                funcs.add(fn_to_py.get(rust_fn, rust_fn))

            for em in re.finditer(rf"\b{re.escape(var_name)}\.add::<([A-Za-z_][A-Za-z0-9_]*)>\(", body):
                classes.add(em.group(1))

    return submod_classes, submod_funcs, class_methods, all_pyclasses, all_pyfunctions


def _parse_stub_tree() -> ast.Module:
    """Parse `__init__.pyi` with `ast.parse`."""
    return ast.parse(INIT_PYI.read_text(encoding="utf-8"), filename=str(INIT_PYI))


def _parse_stub_declarations() -> tuple[set[str], set[str], set[str]]:
    """Return `(class_names, func_names, ann_names)` from top-level AST of `__init__.pyi`."""
    tree = _parse_stub_tree()
    classes: set[str] = set()
    funcs: set[str] = set()
    ann_vars: set[str] = set()
    for node in tree.body:
        if isinstance(node, ast.ClassDef):
            classes.add(node.name)
        elif isinstance(node, ast.FunctionDef):
            funcs.add(node.name)
        elif isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name):
            ann_vars.add(node.target.id)
    return classes, funcs, ann_vars


def _parse_stub_submodule_details() -> tuple[
    dict[str, set[str]],
    dict[str, set[str]],
    dict[str, set[str]],
    dict[str, set[str]],
    list[tuple[str, int]],
]:
    """Return `(submod_classes, submod_funcs, submod_attrs, class_methods, missing_return_annotations)` from `__init__.pyi`."""
    tree = _parse_stub_tree()
    submod_classes: dict[str, set[str]] = {}
    submod_funcs: dict[str, set[str]] = {}
    submod_attrs: dict[str, set[str]] = {}
    class_methods: dict[str, set[str]] = {}

    for node in tree.body:
        if isinstance(node, ast.ClassDef):
            classes: set[str] = set()
            funcs: set[str] = set()
            attrs: set[str] = set()
            for member in node.body:
                if isinstance(member, ast.ClassDef):
                    classes.add(member.name)
                    class_methods[member.name] = {
                        m.name for m in member.body if isinstance(m, ast.FunctionDef)
                    }
                elif isinstance(member, ast.FunctionDef):
                    funcs.add(member.name)
                elif isinstance(member, ast.AnnAssign) and isinstance(member.target, ast.Name):
                    attrs.add(member.target.id)
            submod_classes[node.name] = classes
            submod_funcs[node.name] = funcs
            submod_attrs[node.name] = attrs

    missing_returns: list[tuple[str, int]] = []
    for node in ast.walk(tree):
        if isinstance(node, ast.FunctionDef) and node.returns is None:
            missing_returns.append((node.name, node.lineno))

    return submod_classes, submod_funcs, submod_attrs, class_methods, missing_returns


def _parse_init_exports() -> tuple[set[str], set[str], set[str]]:
    """Return `(imported_from_ext, top_level_funcs, dunder_all)` from `__init__.py`."""
    tree = ast.parse(INIT_PY.read_text(encoding="utf-8"), filename=str(INIT_PY))
    imported: set[str] = set()
    funcs: set[str] = set()
    dunder_all: set[str] = set()

    for node in tree.body:
        if isinstance(node, ast.ImportFrom) and node.module == "_nv_rs":
            for alias in node.names:
                imported.add(alias.asname or alias.name)
        elif isinstance(node, ast.FunctionDef):
            funcs.add(node.name)
        elif isinstance(node, ast.Assign):
            for target in node.targets:
                if isinstance(target, ast.Name) and target.id == "__all__":
                    if isinstance(node.value, (ast.List, ast.Tuple)):
                        for elt in node.value.elts:
                            if isinstance(elt, ast.Constant) and isinstance(elt.value, str):
                                dunder_all.add(elt.value)

    return imported, funcs, dunder_all


def _load_pyproject() -> dict:
    text = PYPROJECT_TOML.read_text(encoding="utf-8")
    if tomllib is not None:
        return tomllib.loads(text)
    # Minimal fallback for Python 3.10 when tomllib is absent.
    name_m = re.search(r'(?m)^name\s*=\s*"([^"]+)"', text)
    desc_m = re.search(r'(?m)^description\s*=\s*"([^"]+)"', text)
    mod_m = re.search(r'(?m)^module-name\s*=\s*"([^"]+)"', text)
    inc_m = re.search(r'(?ms)^include\s*=\s*\[(.*?)\]', text)
    includes = re.findall(r'"([^"]+)"', inc_m.group(1)) if inc_m else []
    return {
        "project": {
            "name": name_m.group(1) if name_m else "",
            "description": desc_m.group(1) if desc_m else "",
        },
        "tool": {
            "maturin": {
                "module-name": mod_m.group(1) if mod_m else "",
                "include": includes,
            }
        },
    }


class TestStubSync(unittest.TestCase):
    def test_rust_submodules_match_pyi_stub(self) -> None:
        rust_submodules, rust_attrs = _parse_rust_registered_submodules()
        stub_classes, _, stub_ann_vars = _parse_stub_declarations()

        missing_in_stub = sorted(rust_submodules - stub_classes)
        extra_in_stub = sorted(stub_classes - rust_submodules)
        self.assertEqual(missing_in_stub, [], f"Submodules registered in Rust but missing in __init__.pyi: {missing_in_stub}")
        self.assertEqual(extra_in_stub, [], f"Submodules declared in __init__.pyi but not registered in Rust: {extra_in_stub}")

        for attr in rust_attrs:
            self.assertIn(attr, stub_ann_vars, f"Top-level Rust attr `{attr}` missing from __init__.pyi")

    def test_rust_submodules_and_helpers_match_init_py(self) -> None:
        rust_submodules, rust_attrs = _parse_rust_registered_submodules()
        _, stub_funcs, _ = _parse_stub_declarations()
        imported, init_funcs, dunder_all = _parse_init_exports()

        expected_from_ext = rust_submodules | rust_attrs
        missing_imports = sorted(expected_from_ext - imported)
        extra_imports = sorted(imported - expected_from_ext)
        self.assertEqual(missing_imports, [], f"Missing `_nv_rs` imports in __init__.py: {missing_imports}")
        self.assertEqual(extra_imports, [], f"Extra `_nv_rs` imports in __init__.py: {extra_imports}")

        self.assertEqual(init_funcs, stub_funcs, "Top-level functions in __init__.py and __init__.pyi must match")

        expected_all = expected_from_ext | init_funcs
        self.assertEqual(dunder_all, expected_all, "__all__ in __init__.py must match exported symbols")

    def test_pyproject_metadata_and_stub_packaging(self) -> None:
        data = _load_pyproject()
        project = data.get("project", {})
        maturin = data.get("tool", {}).get("maturin", {})

        self.assertEqual(project.get("name"), "nv-rs")
        self.assertTrue(project.get("description"), "pyproject.toml project.description must be non-empty")
        self.assertEqual(maturin.get("module-name"), "nv_rs._nv_rs")

        includes = set(maturin.get("include", []))
        self.assertIn("python/nv_rs/py.typed", includes)
        self.assertIn("python/nv_rs/__init__.pyi", includes)
        self.assertTrue(PY_TYPED.is_file(), f"Missing {PY_TYPED}")
        self.assertTrue(INIT_PYI.is_file(), f"Missing {INIT_PYI}")

    def test_rust_pyclasses_pyfunctions_and_methods_match_pyi(self) -> None:
        rust_submodules, _ = _parse_rust_registered_submodules()
        (
            rust_classes,
            rust_funcs,
            rust_methods,
            all_pyclasses,
            all_pyfunctions,
        ) = _parse_rust_submodule_items()
        pyi_classes, pyi_funcs, _, pyi_methods, _ = _parse_stub_submodule_details()

        all_pyi_classes = set().union(*pyi_classes.values()) if pyi_classes else set()
        all_pyi_funcs = set().union(*pyi_funcs.values()) if pyi_funcs else set()

        self.assertEqual(
            sorted(all_pyclasses - all_pyi_classes),
            [],
            "Every #[pyclass] in Rust must be declared in __init__.pyi",
        )
        self.assertEqual(
            sorted(all_pyfunctions - all_pyi_funcs),
            [],
            "Every #[pyfunction] in Rust must be declared in __init__.pyi",
        )

        for submod in sorted(rust_submodules):
            self.assertIn(submod, pyi_classes, f"Missing submodule `{submod}` in __init__.pyi")
            missing_cls = sorted(rust_classes.get(submod, set()) - pyi_classes[submod])
            self.assertEqual(
                missing_cls,
                [],
                f"Submodule `{submod}` missing Rust classes in __init__.pyi: {missing_cls}",
            )
            missing_fns = sorted(rust_funcs.get(submod, set()) - pyi_funcs[submod])
            self.assertEqual(
                missing_fns,
                [],
                f"Submodule `{submod}` missing Rust functions in __init__.pyi: {missing_fns}",
            )

        for cls_name, expected_methods in sorted(rust_methods.items()):
            self.assertIn(cls_name, pyi_methods, f"Missing class `{cls_name}` in __init__.pyi")
            # `__repr__` is inherited from `object` and intentionally omitted from stubs.
            missing_methods = sorted((expected_methods - {"__repr__"}) - pyi_methods[cls_name])
            self.assertEqual(
                missing_methods,
                [],
                f"Class `{cls_name}` missing #[pymethods] in __init__.pyi: {missing_methods}",
            )

    def test_stub_functions_have_return_annotations(self) -> None:
        _, _, _, _, missing_returns = _parse_stub_submodule_details()
        self.assertEqual(
            missing_returns,
            [],
            f"Functions/methods in __init__.pyi missing return annotations: {missing_returns}",
        )

    def test_runtime_inspect_submodule_parity(self) -> None:
        try:
            import nv_rs._nv_rs as ext
        except ImportError:
            try:
                import pytest

                pytest.skip("compiled _nv_rs extension not installed")
            except ImportError:
                self.skipTest("compiled _nv_rs extension not installed")

        import inspect

        rust_submodules, _ = _parse_rust_registered_submodules()
        pyi_classes, pyi_funcs, pyi_attrs, _, _ = _parse_stub_submodule_details()

        for submod in sorted(rust_submodules):
            self.assertTrue(hasattr(ext, submod), f"Runtime `_nv_rs` missing submodule `{submod}`")
            mod_obj = getattr(ext, submod)
            runtime_public = {
                name
                for name, _ in inspect.getmembers(mod_obj)
                if not name.startswith("_")
            }
            stub_public = pyi_classes[submod] | pyi_funcs[submod] | pyi_attrs[submod]
            self.assertEqual(
                runtime_public,
                stub_public,
                f"Runtime `_nv_rs.{submod}` public names do not match __init__.pyi",
            )


if __name__ == "__main__":
    unittest.main()
