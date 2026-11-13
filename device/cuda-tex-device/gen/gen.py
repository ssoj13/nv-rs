import re
import sys

OUT = sys.argv[1]
# (rust name, CUDA name, ptx op prefix, coord type, params [(name, rust type)], asm ins [(constraint, expr)],
#  operand tail after "{%0, %1, %2, %3}, ", shape text)
F, I, U = "f32", "i32", "u32"
G2, G3 = "[f32; 2]", "[f32; 3]"
OPS = [
 ("tex1d", "tex1D", "tex.1d.v4", "f32", [("x",F)], [("f","x")], "[%4, {%5}]", "1D"),
 ("tex1d_fetch", "tex1Dfetch", "tex.1d.v4", "s32", [("x",I)], [("r","x")], "[%4, {%5}]", "linear-memory (`CU_RESOURCE_TYPE_LINEAR`)"),
 ("tex1d_lod", "tex1DLod", "tex.level.1d.v4", "f32", [("x",F),("level",F)], [("f","x"),("f","level")], "[%4, {%5}], %6", "1D (mipmapped)"),
 ("tex1d_grad", "tex1DGrad", "tex.grad.1d.v4", "f32", [("x",F),("dpdx",F),("dpdy",F)], [("f","x"),("f","dpdx"),("f","dpdy")], "[%4, {%5}], {%6}, {%7}", "1D (mipmapped)"),
 ("tex1d_layered", "tex1DLayered", "tex.a1d.v4", "f32", [("x",F),("layer",U)], [("r","layer"),("f","x")], "[%4, {%5, %6}]", "1D layered"),
 ("tex1d_layered_lod", "tex1DLayeredLod", "tex.level.a1d.v4", "f32", [("x",F),("layer",U),("level",F)], [("r","layer"),("f","x"),("f","level")], "[%4, {%5, %6}], %7", "1D layered (mipmapped)"),
 ("tex1d_layered_grad", "tex1DLayeredGrad", "tex.grad.a1d.v4", "f32", [("x",F),("layer",U),("dpdx",F),("dpdy",F)], [("r","layer"),("f","x"),("f","dpdx"),("f","dpdy")], "[%4, {%5, %6}], {%7}, {%8}", "1D layered (mipmapped)"),
 ("tex2d", "tex2D", "tex.2d.v4", "f32", [("x",F),("y",F)], [("f","x"),("f","y")], "[%4, {%5, %6}]", "2D (array or pitched)"),
 ("tex2d_lod", "tex2DLod", "tex.level.2d.v4", "f32", [("x",F),("y",F),("level",F)], [("f","x"),("f","y"),("f","level")], "[%4, {%5, %6}], %7", "2D (mipmapped)"),
 ("tex2d_grad", "tex2DGrad", "tex.grad.2d.v4", "f32", [("x",F),("y",F),("dpdx",G2),("dpdy",G2)], [("f","x"),("f","y"),("f","dpdx[0]"),("f","dpdx[1]"),("f","dpdy[0]"),("f","dpdy[1]")], "[%4, {%5, %6}], {%7, %8}, {%9, %10}", "2D (mipmapped)"),
 ("tex2d_layered", "tex2DLayered", "tex.a2d.v4", "f32", [("x",F),("y",F),("layer",U)], [("r","layer"),("f","x"),("f","y")], "[%4, {%5, %6, %7, %7}]", "2D layered"),
 ("tex2d_layered_lod", "tex2DLayeredLod", "tex.level.a2d.v4", "f32", [("x",F),("y",F),("layer",U),("level",F)], [("r","layer"),("f","x"),("f","y"),("f","level")], "[%4, {%5, %6, %7, %7}], %8", "2D layered (mipmapped)"),
 ("tex2d_layered_grad", "tex2DLayeredGrad", "tex.grad.a2d.v4", "f32", [("x",F),("y",F),("layer",U),("dpdx",G2),("dpdy",G2)], [("r","layer"),("f","x"),("f","y"),("f","dpdx[0]"),("f","dpdx[1]"),("f","dpdy[0]"),("f","dpdy[1]")], "[%4, {%5, %6, %7, %7}], {%8, %9}, {%10, %11}", "2D layered (mipmapped)"),
 ("tex3d", "tex3D", "tex.3d.v4", "f32", [("x",F),("y",F),("z",F)], [("f","x"),("f","y"),("f","z")], "[%4, {%5, %6, %7, %7}]", "3D"),
 ("tex3d_lod", "tex3DLod", "tex.level.3d.v4", "f32", [("x",F),("y",F),("z",F),("level",F)], [("f","x"),("f","y"),("f","z"),("f","level")], "[%4, {%5, %6, %7, %7}], %8", "3D (mipmapped)"),
 ("tex3d_grad", "tex3DGrad", "tex.grad.3d.v4", "f32", [("x",F),("y",F),("z",F),("dpdx",G3),("dpdy",G3)], [("f","x"),("f","y"),("f","z"),("f","dpdx[0]"),("f","dpdx[1]"),("f","dpdx[2]"),("f","dpdy[0]"),("f","dpdy[1]"),("f","dpdy[2]")], "[%4, {%5, %6, %7, %7}], {%8, %9, %10, %10}, {%11, %12, %13, %13}", "3D (mipmapped)"),
 ("tex_cube", "texCubemap", "tex.cube.v4", "f32", [("x",F),("y",F),("z",F)], [("f","x"),("f","y"),("f","z")], "[%4, {%5, %6, %7, %7}]", "cubemap (`(x, y, z)` is a direction)"),
 ("tex_cube_lod", "texCubemapLod", "tex.level.cube.v4", "f32", [("x",F),("y",F),("z",F),("level",F)], [("f","x"),("f","y"),("f","z"),("f","level")], "[%4, {%5, %6, %7, %7}], %8", "cubemap (mipmapped)"),
 ("tex_cube_grad", "texCubemapGrad", "tex.grad.cube.v4", "f32", [("x",F),("y",F),("z",F),("dpdx",G3),("dpdy",G3)], [("f","x"),("f","y"),("f","z"),("f","dpdx[0]"),("f","dpdx[1]"),("f","dpdx[2]"),("f","dpdy[0]"),("f","dpdy[1]"),("f","dpdy[2]")], "[%4, {%5, %6, %7, %7}], {%8, %9, %10, %10}, {%11, %12, %13, %13}", "cubemap (mipmapped)"),
 ("tex_cube_layered", "texCubemapLayered", "tex.acube.v4", "f32", [("x",F),("y",F),("z",F),("layer",U)], [("r","layer"),("f","x"),("f","y"),("f","z")], "[%4, {%5, %6, %7, %8}]", "cubemap layered"),
 ("tex_cube_layered_lod", "texCubemapLayeredLod", "tex.level.acube.v4", "f32", [("x",F),("y",F),("z",F),("layer",U),("level",F)], [("r","layer"),("f","x"),("f","y"),("f","z"),("f","level")], "[%4, {%5, %6, %7, %8}], %9", "cubemap layered (mipmapped)"),
 ("tex_cube_layered_grad", "texCubemapLayeredGrad", "tex.grad.acube.v4", "f32", [("x",F),("y",F),("z",F),("layer",U),("dpdx",G3),("dpdy",G3)], [("r","layer"),("f","x"),("f","y"),("f","z"),("f","dpdx[0]"),("f","dpdx[1]"),("f","dpdx[2]"),("f","dpdy[0]"),("f","dpdy[1]"),("f","dpdy[2]")], "[%4, {%5, %6, %7, %8}], {%9, %10, %11, %11}, {%12, %13, %14, %14}", "cubemap layered (mipmapped)"),
]
OPMAP = {op[0]: op for op in OPS}
# CUDA's `bool* isResident` overloads (texture_indirect_functions.h, under __NV_TEX_SPARSE).
SPARSE = ["tex2d", "tex3d", "tex2d_layered", "tex2d_lod", "tex3d_lod", "tex2d_layered_lod",
          "tex2d_grad", "tex3d_grad", "tex2d_layered_grad"]
GATHER = [("gather_r","tld4.r.2d.v4"),("gather_g","tld4.g.2d.v4"),("gather_b","tld4.b.2d.v4"),("gather_a","tld4.a.2d.v4")]
GATHER_ARGS = ([("x",F),("y",F)], [("f","x"),("f","y")], "[%4, {%5, %6}]")
SCALARS = [("f32","f32","=f"),("i32","s32","=r"),("u32","u32","=r")]


def params(ps): return "".join(f", {n}: {t}" for n,t in ps)
def names(ps): return "".join(f", {n}" for n,_ in ps)
def shift(tail): return re.sub(r"%(\d+)", lambda m: "%" + str(int(m.group(1)) + (1 if int(m.group(1)) >= 4 else 0)), tail)


o = []
w = o.append


def scalar_method(name, op, ctype, ps, ins, tail, rt, dt, oc, sparse):
    instr = f"{op}.{dt}.{ctype}"
    ins_s = ", ".join(f'in("{c}") {e}' for c,e in ins)
    outs = f'out("{oc}") a, out("{oc}") b, out("{oc}") c, out("{oc}") d'
    if sparse:
        w(f"    /// PTX `{instr}` with its residency predicate (`selp.u16`).")
        w("    #[inline(always)]")
        w(f"    unsafe fn {name}_sparse(t: u64{params(ps)}) -> ([{rt}; 4], bool) {{")
        w(f"        let (a, b, c, d): ({rt}, {rt}, {rt}, {rt});")
        w("        let r: u16;")
        w("        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.")
        tmpl = f"{{.reg .pred %%p0;\\n\\t{instr} {{%0, %1, %2, %3}}|%%p0, {shift(tail)};\\n\\tselp.u16 %4, 1, 0, %%p0; }}"
        w(f'        unsafe {{ ptx_asm!("{tmpl}", {outs}, out("=h") r, in("l") t, {ins_s}, options(register_only)) }};')
        w("        ([a, b, c, d], r != 0)")
        w("    }")
    else:
        w(f"    /// PTX `{instr}`.")
        w("    #[inline(always)]")
        w(f"    unsafe fn {name}(t: u64{params(ps)}) -> [{rt}; 4] {{")
        w(f"        let (a, b, c, d): ({rt}, {rt}, {rt}, {rt});")
        w("        // SAFETY: a register-only texture fetch; the caller guarantees `t` is a live texture of this shape.")
        w(f'        unsafe {{ ptx_asm!("{instr} {{%0, %1, %2, %3}}, {tail};", {outs}, in("l") t, {ins_s}, options(register_only)) }};')
        w("        [a, b, c, d]")
        w("    }")


TRAIT = []
for (n,_,op,ct,ps,ins,tail,_) in OPS:
    TRAIT.append(f"        /// Four channels of `{op}` for this scalar.\n        unsafe fn {n}(t: u64{params(ps)}) -> [Self; 4];")
for n in SPARSE:
    op = OPMAP[n]
    TRAIT.append(f"        /// Four channels of `{op[2]}` and whether they are resident.\n        unsafe fn {n}_sparse(t: u64{params(op[4])}) -> ([Self; 4], bool);")
for (n,op) in GATHER:
    TRAIT.append(f"        /// One channel of four texels (`{op}`) for this scalar.\n        unsafe fn {n}(t: u64, x: f32, y: f32) -> [Self; 4];")
    TRAIT.append(f"        /// As `{n}`, and whether they are resident.\n        unsafe fn {n}_sparse(t: u64, x: f32, y: f32) -> ([Self; 4], bool);")

IMPLS = []
for rt,dt,oc in SCALARS:
    o.clear()
    w(f"impl sealed::Scalar for {rt} {{")
    for (n,_,op,ct,ps,ins,tail,_) in OPS:
        scalar_method(n, op, ct, ps, ins, tail, rt, dt, oc, False)
    for n in SPARSE:
        (_,_,op,ct,ps,ins,tail,_) = OPMAP[n]
        scalar_method(n, op, ct, ps, ins, tail, rt, dt, oc, True)
    for (n,op) in GATHER:
        ps, ins, tail = GATHER_ARGS
        scalar_method(n, op, "f32", ps, ins, tail, rt, dt, oc, False)
        scalar_method(n, op, "f32", ps, ins, tail, rt, dt, oc, True)
    w("}")
    IMPLS.append("\n".join(o))

TEX_SAFETY = [
    "/// # Safety",
    "///",
]

o.clear()
for (n,cn,op,ct,ps,ins,tail,shape) in OPS:
    w(f"/// `{cn}<T>`: PTX `{op}.{{f32,s32,u32}}.{ct}`, keeping the first channels `T` holds.")
    if n == "tex1d_fetch":
        w("///")
        w("/// `x` is an element index; the fetch is unfiltered and unnormalized.")
    w("///")
    w("/// # Safety")
    w("///")
    w(f"/// `tex` must be a live texture object over a {shape} resource.")
    w("/// [`TexElem`] says which `T` matches its read mode. Valid in any kernel, device function or")
    w("/// OptiX program.")
    w("#[inline(always)]")
    w(f"pub unsafe fn {n}<T: TexElem>(tex: TexObject{params(ps)}) -> T {{")
    w("    // SAFETY: forwarded contract.")
    w(f"    T::from_v4(unsafe {{ <T::S as Scalar>::{n}(tex{names(ps)}) }})")
    w("}")
    w("")
for n in SPARSE:
    (_,cn,op,ct,ps,ins,tail,shape) = OPMAP[n]
    w(f"/// `{cn}<T>(..., bool* isResident)`: [`{n}`] plus whether every texel the fetch touches is")
    w(f"/// resident (PTX `{op}.{{f32,s32,u32}}.{ct}` with a `|p` predicate destination).")
    w("///")
    w("/// # Safety")
    w("///")
    w(f"/// As [`{n}`]; sm_60 or later (CUDA's `__NV_TEX_SPARSE`).")
    w("#[inline(always)]")
    w(f"pub unsafe fn {n}_sparse<T: TexElem>(tex: TexObject{params(ps)}) -> (T, bool) {{")
    w("    // SAFETY: forwarded contract.")
    w(f"    let (v, r) = unsafe {{ <T::S as Scalar>::{n}_sparse(tex{names(ps)}) }};")
    w("    (T::from_v4(v), r)")
    w("}")
    w("")
TEXFNS = "\n".join(o).rstrip("\n")

o.clear()
for sparse in (False, True):
    sfx = "_sparse" if sparse else ""
    ret = "(T, bool)" if sparse else "T"
    if sparse:
        w("/// `tex2Dgather<T>(tex, x, y, &isResident, COMP)`: [`tex2d_gather`] plus whether every texel")
        w("/// is resident (PTX `tld4.{r,g,b,a}.2d.v4.{f32,s32,u32}.f32` with a `|p` predicate destination).")
        w("///")
        w("/// # Safety")
        w("///")
        w("/// As [`tex2d_gather`]; sm_60 or later (CUDA's `__NV_TEX_SPARSE`).")
    else:
        w("/// `tex2Dgather<T>(tex, x, y, COMP)`: PTX `tld4.{r,g,b,a}.2d.v4.{f32,s32,u32}.f32` — channel")
        w("/// `COMP` (0 = r, 1 = g, 2 = b, 3 = a) of the four texels bilinear filtering at `(x, y)` would")
        w("/// use, in the order of CUDA's `tex2Dgather`.")
        w("///")
        w("/// # Safety")
        w("///")
        w("/// `tex` must be a live texture object over a 2D resource; [`TexElem`] says which `T` matches")
        w("/// its read mode. Valid in any kernel, device function or OptiX program.")
    w("#[inline(always)]")
    w(f"pub unsafe fn tex2d_gather{sfx}<T: TexElem, const COMP: u32>(tex: TexObject, x: f32, y: f32) -> {ret} {{")
    w(f'    const {{ assert!(COMP < 4, "tex2d_gather{sfx}: COMP selects r, g, b or a (0..=3)") }};')
    w("    // SAFETY: forwarded contract; `COMP` is a constant, so one arm survives.")
    lhs = "let (v, r) = unsafe {" if sparse else "T::from_v4(unsafe {"
    w(f"    {lhs}")
    w("        match COMP {")
    for i, (g, _) in enumerate(GATHER):
        pat = str(i) if i < 3 else "_"
        w(f"            {pat} => <T::S as Scalar>::{g}{sfx}(tex, x, y),")
    w("        }")
    if sparse:
        w("    };")
        w("    (T::from_v4(v), r)")
    else:
        w("    })")
    w("}")
    w("")
GATHERFNS = "\n".join(o).rstrip("\n")

# ---------------- surfaces ----------------
SURF = [
 ("1d", [("x",I)], ["x"], "{{%{b}}}"),
 ("2d", [("x",I),("y",I)], ["x","y"], "{{%{b}, %{b1}}}"),
 ("3d", [("x",I),("y",I),("z",I)], ["x","y","z"], "{{%{b}, %{b1}, %{b2}, %{b2}}}"),
 ("a1d", [("x",I),("layer",U)], ["x","layer"], "{{%{b1}, %{b}}}"),
 ("a2d", [("x",I),("y",I),("layer",U)], ["x","y","layer"], "{{%{b2}, %{b}, %{b1}, %{b1}}}"),
]
# (bits, components); PTX allows .v4 only up to .b32.
WIDTHS = [(8,1),(8,2),(8,4),(16,1),(16,2),(16,4),(32,1),(32,2),(32,4),(64,1),(64,2)]
REG = {8: ("u16","h"), 16: ("u16","h"), 32: ("u32","r"), 64: ("u64","l")}
MODES = [("Trap","trap"),("Clamp","clamp"),("Zero","zero")]
VARS = ["a","b","c","d"]


def ptype(bits, n): return (f"v{n}." if n > 1 else "") + f"b{bits}"


o.clear()
for key,ps,order,ctmpl in SURF:
    k = len(ps)
    ins_s = ", ".join(f'in("r") {e}' for e in order)
    for bits, n in WIDTHS:
        rt, rc = REG[bits]
        ty = ptype(bits, n)
        vars_ = VARS[:n]
        arr = f"[{rt}; {n}]"
        # read: outputs %0..n-1, surface %n, coordinates from %n+1
        outs = ", ".join(f"%{i}" for i in range(n))
        coords = ctmpl.format(b=n+1, b1=n+2, b2=n+3)
        outs_s = ", ".join(f'out("={rc}") {v}' for v in vars_)
        w(f"/// PTX `suld.b.{key}.{ty}.{{trap,clamp,zero}}`.")
        w("///")
        w("/// # Safety")
        w("///")
        w("/// A live surface object of this shape whose element has this layout.")
        w("#[inline(always)]")
        w(f"unsafe fn suld_{key}_b{bits}_{n}(s: u64{params(ps)}, m: Boundary) -> {arr} {{")
        w("    let " + (f"a: {rt}" if n == 1 else "(" + ", ".join(vars_) + "): (" + ", ".join([rt]*n) + ")") + ";")
        w("    // SAFETY: forwarded contract. Deliberately not register-only: the load must stay ordered")
        w("    // after this thread's earlier `sust` stores to the same surface.")
        w("    unsafe {")
        w("        match m {")
        for mv, ms in MODES:
            w(f'            Boundary::{mv} => ptx_asm!("suld.b.{key}.{ty}.{ms} {{{outs}}}, [%{n}, {coords}];", {outs_s}, in("l") s, {ins_s}),')
        w("        }")
        w("    }")
        w("    [" + ", ".join(vars_) + "]")
        w("}")
        w("")
        # write: surface %0, coordinates %1.., data after
        coords = ctmpl.format(b=1, b1=2, b2=3)
        data = ", ".join(f"%{1+k+i}" for i in range(n))
        vin = ", ".join(f'in("{rc}") v[{i}]' for i in range(n))
        w(f"/// PTX `sust.b.{key}.{ty}.{{trap,clamp,zero}}`.")
        w("///")
        w("/// # Safety")
        w("///")
        w("/// A live surface object of this shape whose element has this layout.")
        w("#[inline(always)]")
        w(f"unsafe fn sust_{key}_b{bits}_{n}(v: {arr}, s: u64{params(ps)}, m: Boundary) {{")
        w("    // SAFETY: forwarded contract.")
        w("    unsafe {")
        w("        match m {")
        for mv, ms in MODES:
            w(f'            Boundary::{mv} => ptx_asm!("sust.b.{key}.{ty}.{ms} [%0, {coords}], {{{data}}};", in("l") s, {ins_s}, {vin}),')
        w("        }")
        w("    }")
        w("}")
        w("")
SURFRAW = "\n".join(o)

PUB_SURF = [
 ("surf1d", "surf1D", "1d", [("x",I)], "1D"),
 ("surf2d", "surf2D", "2d", [("x",I),("y",I)], "2D"),
 ("surf3d", "surf3D", "3d", [("x",I),("y",I),("z",I)], "3D"),
 ("surf1d_layered", "surf1DLayered", "a1d", [("x",I),("layer",U)], "1D layered"),
 ("surf2d_layered", "surf2DLayered", "a2d", [("x",I),("y",I),("layer",U)], "2D layered"),
 ("surf_cube", "surfCubemap", "a2d", [("x",I),("y",I),("face",U)], "cubemap (`face` in `0..6`)"),
 ("surf_cube_layered", "surfCubemapLayered", "a2d", [("x",I),("y",I),("layer_face",U)], "cubemap layered (`layer_face` = layer * 6 + face)"),
]


def widen(rt, n):
    vals = [("" if rt == "u64" else "u64::from(") + v + ("" if rt == "u64" else ")") for v in VARS[:n]]
    return "[" + ", ".join(vals + ["0"] * (4 - n)) + "]"


o.clear()
for nm,cn,key,ps,shape in PUB_SURF:
    w(f"/// `{cn}read<T>(surf, ..., mode)`: PTX `suld.b.{key}.{{b8..v4.b32,b64,v2.b64}}.{{trap,clamp,zero}}`.")
    w("///")
    w("/// # Safety")
    w("///")
    w(f"/// `surf` must be a live surface object over a {shape} array.")
    w("/// Its element must have `T`'s size and `x` is in bytes (see [`SurfElem`]). With")
    w("/// [`Boundary::Trap`] an out-of-range coordinate aborts the launch.")
    w("#[inline(always)]")
    w(f"pub unsafe fn {nm}_read_mode<T: SurfElem>(surf: SurfObject{params(ps)}, mode: Boundary) -> T {{")
    w("    // SAFETY: forwarded contract; `T::BITS` and `T::N` are constants, so one arm survives.")
    w("    let w = unsafe {")
    w("        match (T::BITS, T::N) {")
    for idx, (bits, n) in enumerate(WIDTHS):
        rt, _ = REG[bits]
        last = idx == len(WIDTHS) - 1
        pat = "_" if last else f"({bits}, {n})"
        w(f"            {pat} => {{")
        w(f"                let [{', '.join(VARS[:n])}] = suld_{key}_b{bits}_{n}(surf{names(ps)}, mode);")
        w(f"                {widen(rt, n)}")
        w("            }")
    w("        }")
    w("    };")
    w("    T::from_bits(w)")
    w("}")
    w("")
    w(f"/// `{cn}read<T>(surf, ...)`: [`{nm}_read_mode`] with CUDA's default [`Boundary::Trap`].")
    w("///")
    w("/// # Safety")
    w("///")
    w(f"/// As [`{nm}_read_mode`].")
    w("#[inline(always)]")
    w(f"pub unsafe fn {nm}_read<T: SurfElem>(surf: SurfObject{params(ps)}) -> T {{")
    w("    // SAFETY: forwarded contract.")
    w(f"    unsafe {{ {nm}_read_mode(surf{names(ps)}, Boundary::Trap) }}")
    w("}")
    w("")
    w(f"/// `{cn}write<T>(v, surf, ..., mode)`: PTX `sust.b.{key}.{{b8..v4.b32,b64,v2.b64}}.{{trap,clamp,zero}}`.")
    w("///")
    w("/// # Safety")
    w("///")
    w(f"/// As [`{nm}_read_mode`].")
    w("#[inline(always)]")
    w(f"pub unsafe fn {nm}_write_mode<T: SurfElem>(v: T, surf: SurfObject{params(ps)}, mode: Boundary) {{")
    w("    let w = v.to_bits();")
    w("    // SAFETY: forwarded contract; `T::BITS` and `T::N` are constants, so one arm survives; the")
    w("    // narrowing casts keep exactly the component bits `to_bits` zero-extended.")
    w("    unsafe {")
    w("        match (T::BITS, T::N) {")
    for idx, (bits, n) in enumerate(WIDTHS):
        rt, _ = REG[bits]
        last = idx == len(WIDTHS) - 1
        pat = "_" if last else f"({bits}, {n})"
        conv = (lambda i: f"w[{i}]") if rt == "u64" else (lambda i, rt=rt: f"w[{i}] as {rt}")
        arr = "[" + ", ".join(conv(i) for i in range(n)) + "]"
        w(f"            {pat} => sust_{key}_b{bits}_{n}({arr}, surf{names(ps)}, mode),")
    w("        }")
    w("    }")
    w("}")
    w("")
    w(f"/// `{cn}write<T>(v, surf, ...)`: [`{nm}_write_mode`] with CUDA's default [`Boundary::Trap`].")
    w("///")
    w("/// # Safety")
    w("///")
    w(f"/// As [`{nm}_read_mode`].")
    w("#[inline(always)]")
    w(f"pub unsafe fn {nm}_write<T: SurfElem>(v: T, surf: SurfObject{params(ps)}) {{")
    w("    // SAFETY: forwarded contract.")
    w(f"    unsafe {{ {nm}_write_mode(v, surf{names(ps)}, Boundary::Trap) }}")
    w("}")
    w("")
PUBSURF = "\n".join(o).rstrip("\n") + "\n"

# Surface element impls: (type, bits, to-bits expr, from-bits expr, component counts).
SELEMS = [
 ("i8", 8, "u64::from({v} as u8)", "{w} as u8 as i8", [1,2,4]),
 ("u8", 8, "u64::from({v})", "{w} as u8", [1,2,4]),
 ("i16", 16, "u64::from({v} as u16)", "{w} as u16 as i16", [1,2,4]),
 ("u16", 16, "u64::from({v})", "{w} as u16", [1,2,4]),
 ("i32", 32, "u64::from({v} as u32)", "{w} as u32 as i32", [1,2,4]),
 ("u32", 32, "u64::from({v})", "{w} as u32", [1,2,4]),
 ("f32", 32, "u64::from({v}.to_bits())", "f32::from_bits({w} as u32)", [1,2,4]),
 ("i64", 64, "{v} as u64", "{w} as i64", [1,2]),
 ("u64", 64, "{v}", "{w}", [1,2]),
 ("f64", 64, "{v}.to_bits()", "f64::from_bits({w})", [1,2]),
]
o.clear()
for s, bits, to, frm, ns in SELEMS:
    for n in ns:
        ty = s if n == 1 else f"[{s}; {n}]"
        if n == 1:
            tob = "[" + to.format(v="self") + ", 0, 0, 0]"
            fromb = frm.format(w="w[0]")
        else:
            tob = "[" + ", ".join([to.format(v=f"self[{i}]") for i in range(n)] + ["0"] * (4 - n)) + "]"
            fromb = "[" + ", ".join(frm.format(w=f"w[{i}]") for i in range(n)) + "]"
        w(f"impl SurfBits for {ty} {{")
        w(f"    const BITS: u32 = {bits};")
        w(f"    const N: usize = {n};")
        w("    #[inline(always)]")
        w("    fn to_bits(self) -> [u64; 4] {")
        w(f"        {tob}")
        w("    }")
        w("    #[inline(always)]")
        w("    fn from_bits(w: [u64; 4]) -> Self {")
        w(f"        {fromb}")
        w("    }")
        w("}")
        w(f"impl SurfElem for {ty} {{}}")
        w("")
SURFELEMS = "\n".join(o).rstrip("\n")

HEAD = open(sys.argv[2], encoding="utf-8").read()
body = (HEAD.replace("@@TRAIT@@", "\n".join(TRAIT)).replace("@@IMPLS@@", "\n\n".join(IMPLS))
        .replace("@@TEXFNS@@", TEXFNS).replace("@@GATHER@@", GATHERFNS).replace("@@SURFRAW@@", SURFRAW)
        .replace("@@PUBSURF@@", PUBSURF).replace("@@SURFELEMS@@", SURFELEMS))
open(OUT, "w", newline="\n", encoding="utf-8").write(body)
