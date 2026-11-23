//! Proc macros for `optix-device`: calls to OptiX direct and continuation callable programs.
//!
//! **Why a macro.** `optixDirectCall` / `optixContinuationCall` resolve an SBT index to a function
//! address (`_optix_call_direct_callable` / `_optix_call_continuation_callable`) and call through
//! it. cuda-oxide does not lower calls through Rust function pointers, so the call is written in
//! PTX: an indirect `call` with a `.callprototype` matching the callee. The prototype depends on
//! the signature, and `ptx_asm!` needs its template as one string literal, so the macro builds
//! that literal per call site.
//! **What it emits.** `{ let f = raw::call_*_callable(index); ptx_asm!("{ .param ...; st.param ...;
//! proto: .callprototype ...; call (ret), %f, (args), proto; ld.param ... }", ...) }` inside an
//! `unsafe` block the caller must already be in (the macros are used from `unsafe` code).
//! **Signatures:** scalars (`u16 i16 u32 i32 f32 u64 i64 f64 usize isize`) and raw pointers, any
//! number of arguments, an optional scalar return — the callee is a `#[cuda_device::device] fn
//! __direct_callable__name(...)` (or `__continuation_callable__`) with the same signature.
//! 16-bit values cross the call as `.b32`, as NVPTX declares them on the callee (case `callables`).
//! **Requires** the calling crate to depend on `optix-device` and `cuda-device` under those names
//! (every OptiX program crate does, for `#[kernel]`): the expansion names `::optix_device::raw`
//! and `::cuda_device::ptx_asm!`, and `ptx_asm!`'s own expansion names `cuda_device::ptx`, so a
//! re-export through `optix_device` would not remove the `cuda-device` dependency.
//! **Used by** `optix_device::{direct_call, continuation_call}`.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Expr, ReturnType, Token, Type, TypeBareFn, parenthesized};

/// `index, fn(T, ...) -> R, (arg, ...)`
struct Call {
    index: Expr,
    sig: TypeBareFn,
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for Call {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let index = input.parse()?;
        input.parse::<Token![,]>()?;
        let sig = input.parse()?;
        input.parse::<Token![,]>()?;
        let inner;
        parenthesized!(inner in input);
        let args = inner.parse_terminated(Expr::parse, Token![,])?;
        if !input.is_empty() {
            input.parse::<Token![,]>()?;
        }
        Ok(Self { index, sig, args })
    }
}

/// Whether a scalar type is 16 bits wide. NVPTX declares 16-bit parameters and results of a
/// `.func` as `.param .b32` (it promotes integers narrower than 32 bits), so the call site must
/// pass them as `.b32` too: a `.b16` prototype mismatches the callee and the value arrives wrong.
fn is_16bit(ty: &Type) -> bool {
    matches!(ty, Type::Path(p) if p.qself.is_none() && p.path.segments.last().is_some_and(|s| s.ident == "u16" || s.ident == "i16"))
}

/// PTX register class of a scalar type as it crosses the call: (constraint letter, bit width).
/// 16-bit types travel in a 32-bit register (see [`is_16bit`]).
fn class(ty: &Type) -> syn::Result<(char, u32)> {
    if let Type::Ptr(_) = ty {
        return Ok(('l', 64));
    }
    let name = match ty {
        Type::Path(p) if p.qself.is_none() => p.path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default(),
        _ => String::new(),
    };
    Ok(match name.as_str() {
        "u16" | "i16" | "u32" | "i32" => ('r', 32),
        "f32" => ('f', 32),
        "u64" | "i64" | "usize" | "isize" => ('l', 64),
        "f64" => ('d', 64),
        _ => return Err(syn::Error::new_spanned(ty, "callable arguments and results must be integer, float or raw-pointer scalars")),
    })
}

fn expand(input: TokenStream, raw_fn: &str) -> TokenStream {
    let call = syn::parse_macro_input!(input as Call);
    match build(&call, raw_fn) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn build(call: &Call, raw_fn: &str) -> syn::Result<proc_macro2::TokenStream> {
    if call.sig.inputs.len() != call.args.len() {
        return Err(syn::Error::new_spanned(&call.sig, format!("signature has {} parameters, {} arguments given", call.sig.inputs.len(), call.args.len())));
    }
    let params: Vec<(char, u32)> = call.sig.inputs.iter().map(|a| class(&a.ty)).collect::<syn::Result<_>>()?;
    let ret = match &call.sig.output {
        ReturnType::Default => None,
        ReturnType::Type(_, t) => Some((class(t)?, t.clone())),
    };
    // Operand numbering: the result (if any) is %0, then the function address, then arguments.
    let first_in = usize::from(ret.is_some());
    let fptr = first_in;
    let mut ptx = String::from("{\n");
    if let Some(((_, bits), _)) = &ret {
        ptx += &format!("\t.param .b{bits} nvrs_rv;\n");
    }
    for (i, (_, bits)) in params.iter().enumerate() {
        ptx += &format!("\t.param .b{bits} nvrs_a{i};\n\tst.param.b{bits} [nvrs_a{i}], %{};\n", fptr + 1 + i);
    }
    let proto_ret = ret.as_ref().map_or(String::new(), |((_, bits), _)| format!("(.param .b{bits} _) "));
    let proto_params = params.iter().map(|(_, bits)| format!(".param .b{bits} _")).collect::<Vec<_>>().join(", ");
    ptx += &format!("\tnvrs_proto: .callprototype {proto_ret}_ ({proto_params});\n");
    let ret_list = if ret.is_some() { "(nvrs_rv), " } else { "" };
    let arg_list = (0..params.len()).map(|i| format!("nvrs_a{i}")).collect::<Vec<_>>().join(", ");
    ptx += &format!("\tcall {ret_list}%{fptr}, ({arg_list}), nvrs_proto;\n");
    if let Some(((_, bits), _)) = &ret {
        ptx += &format!("\tld.param.b{bits} %0, [nvrs_rv];\n");
    }
    ptx += "}";
    let template = syn::LitStr::new(&ptx, Span::call_site());

    let index = &call.index;
    let raw = syn::Ident::new(raw_fn, Span::call_site());
    let args: Vec<proc_macro2::TokenStream> = call
        .args
        .iter()
        .zip(&params)
        .zip(&call.sig.inputs)
        .map(|((a, (c, _)), p)| {
            let c = syn::LitStr::new(&c.to_string(), Span::call_site());
            let ty = &p.ty;
            // `as u32` zero-extends `u16` and sign-extends `i16`, as the callee's ABI expects.
            let widen = is_16bit(ty).then(|| quote! { as u32 });
            quote! { in(#c) { let __nvrs_arg: #ty = #a; __nvrs_arg #widen } }
        })
        .collect();
    Ok(match ret {
        Some(((c, _), ty)) => {
            let c = syn::LitStr::new(&format!("={c}"), Span::call_site());
            // A 16-bit result comes back in a 32-bit register; truncate it to the declared type.
            let (reg_ty, narrow) = if is_16bit(&ty) { (quote! { u32 }, Some(quote! { as #ty })) } else { (quote! { #ty }, None) };
            quote! {{
                let __nvrs_f: u64 = ::optix_device::raw::#raw(#index);
                let __nvrs_r: #reg_ty;
                ::cuda_device::ptx_asm!(#template, out(#c) __nvrs_r, in("l") __nvrs_f, #(#args),*);
                __nvrs_r #narrow
            }}
        }
        None => quote! {{
            let __nvrs_f: u64 = ::optix_device::raw::#raw(#index);
            ::cuda_device::ptx_asm!(#template, in("l") __nvrs_f, #(#args),*);
        }},
    })
}

/// `direct_call!(sbt_index, fn(T, ...) -> R, (args...))` — calls the direct callable at
/// `sbt_index` (`optixDirectCall`). Must be used inside `unsafe`: valid in the program types
/// OptiX allows direct calls from, and the callee must have exactly this signature.
#[proc_macro]
pub fn direct_call(input: TokenStream) -> TokenStream {
    expand(input, "call_direct_callable")
}

/// `continuation_call!(sbt_index, fn(T, ...) -> R, (args...))` — calls the continuation callable
/// at `sbt_index` (`optixContinuationCall`); same contract as [`direct_call!`] for the program
/// types OptiX allows continuation calls from.
#[proc_macro]
pub fn continuation_call(input: TokenStream) -> TokenStream {
    expand(input, "call_continuation_callable")
}
