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
        found = re.findall(r'PyModule::new\s*\(\s*py\s*,\s*"([a-zA-Z_][a-zA-Z0-9_]*)"\s*\)', mod_text)
        if not found:
            raise AssertionError(f"No `PyModule::new(py, \"...\")` found in {mod_path}")
        if "add_submodule" not in mod_text:
            raise AssertionError(f"No `add_submodule` call found in {mod_path}")
        submodules.update(found)

    return submodules, top_attrs


def _parse_stub_declarations() -> tuple[set[str], set[str], set[str]]:
    """Return `(class_names, func_names, ann_names)` from top-level AST of `__init__.pyi`."""
    tree = ast.parse(INIT_PYI.read_text(encoding="utf-8"), filename=str(INIT_PYI))
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


if __name__ == "__main__":
    unittest.main()
