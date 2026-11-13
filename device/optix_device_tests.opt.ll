; ModuleID = 'C:\projects\projects.rust.cg\cglibs\nv-rs\device\optix_device_tests.ll'
source_filename = "optix_device_tests"
target datalayout = "e-p:64:64:64-p3:32:32:32-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64-i128:128:128-f32:32:32-f64:64:64-f128:128:128-v16:16:16-v32:32:32-v64:64:64-v128:128:128-n16:32:64-a:8:8"
target triple = "nvptx64-nvidia-cuda"

@params = addrspace(4) externally_initialized global [24 x i8] zeroinitializer, align 8
@llvm.used = appending global [5 x ptr] [ptr @__closesthit__shade, ptr @__direct_callable__double, ptr @__miss__background, ptr @__raygen__render, ptr addrspacecast (ptr addrspace(4) @params to ptr)], section "llvm.metadata"

; Function Attrs: convergent nounwind
define ptx_kernel void @__raygen__render() #0 {
entry:
  %v6 = tail call i32 asm "call ($0), _optix_get_launch_index_x, ();", "=r"() #2
  %v28 = tail call i32 asm "call ($0), _optix_get_launch_index_y, ();", "=r"() #2
  %v29 = tail call i32 asm "call ($0), _optix_get_launch_index_z, ();", "=r"() #2
  %v35 = load i64, ptr addrspacecast (ptr addrspace(4) @params to ptr), align 8
  %v37 = load i64, ptr getelementptr inbounds nuw (i8, ptr addrspacecast (ptr addrspace(4) @params to ptr), i64 8), align 8
  %v39 = load i32, ptr getelementptr inbounds nuw (i8, ptr addrspacecast (ptr addrspace(4) @params to ptr), i64 16), align 8
  %v41 = load i32, ptr getelementptr inbounds nuw (i8, ptr addrspacecast (ptr addrspace(4) @params to ptr), i64 20), align 4
  %v42 = uitofp i32 %v6 to float
  %v43 = fadd contract float %v42, 5.000000e-01
  %v44 = uitofp i32 %v39 to float
  %v45 = fdiv contract float %v43, %v44
  %v46 = uitofp i32 %v28 to float
  %v47 = fadd contract float %v46, 5.000000e-01
  %v48 = uitofp i32 %v41 to float
  %v49 = fdiv contract float %v47, %v48
  %v99.i = tail call { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } asm sideeffect "call($0,$1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20,$21,$22,$23,$24,$25,$26,$27,$28,$29,$30,$31),_optix_trace_typed_32,($32,$33,$34,$35,$36,$37,$38,$39,$40,$41,$42,$43,$44,$45,$46,$47,$48,$49,$50,$51,$52,$53,$54,$55,$56,$57,$58,$59,$60,$61,$62,$63,$64,$65,$66,$67,$68,$69,$70,$71,$72,$73,$74,$75,$76,$77,$78,$79,$80);", "=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,r,l,f,f,f,f,f,f,f,f,f,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r"(i32 0, i64 %v37, float %v45, float %v49, float -1.000000e+00, float 0.000000e+00, float 0.000000e+00, float 1.000000e+00, float 0.000000e+00, float 1.000000e+16, float 0.000000e+00, i32 255, i32 0, i32 0, i32 1, i32 0, i32 4, i32 0, i32 0, i32 0, i32 -2, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0, i32 0) #2
  %v100.i = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99.i, 0
  %v101.i = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99.i, 1
  %v102.i = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99.i, 2
  %v103.i = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99.i, 3
  %v7 = inttoptr i64 %v35 to ptr
  %v8 = mul i32 %v39, %v28
  %v9 = add i32 %v8, %v6
  %v10 = zext i32 %v9 to i64
  %v11 = getelementptr inbounds nuw [16 x i8], ptr %v7, i64 %v10
  %v23 = sitofp i32 %v103.i to float
  store i32 %v100.i, ptr %v11, align 4
  %v25 = getelementptr inbounds nuw i8, ptr %v11, i64 4
  store i32 %v101.i, ptr %v25, align 4
  %v26 = getelementptr inbounds nuw i8, ptr %v11, i64 8
  store i32 %v102.i, ptr %v26, align 4
  %v27 = getelementptr inbounds nuw i8, ptr %v11, i64 12
  store float %v23, ptr %v27, align 4
  ret void
}

; Function Attrs: convergent nounwind
define ptx_kernel void @__miss__background() #0 {
entry:
  %v1 = tail call i64 asm "call ($0), _optix_get_sbt_data_ptr_64, ();", "=l"() #2
  %v2 = inttoptr i64 %v1 to ptr
  %v4.unpack6 = load i32, ptr %v2, align 4
  %v4.elt1 = getelementptr inbounds nuw i8, ptr %v2, i64 4
  %v4.unpack27 = load i32, ptr %v4.elt1, align 4
  %v4.elt3 = getelementptr inbounds nuw i8, ptr %v2, i64 8
  %v4.unpack48 = load i32, ptr %v4.elt3, align 4
  tail call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 0, i32 %v4.unpack6) #2
  tail call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 1, i32 %v4.unpack27) #2
  tail call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 2, i32 %v4.unpack48) #2
  tail call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 3, i32 -1) #2
  ret void
}

; Function Attrs: convergent nounwind
define ptx_kernel void @__closesthit__shade() #0 {
entry:
  %v2 = tail call i64 asm "call ($0), _optix_get_sbt_data_ptr_64, ();", "=l"() #2
  %v23 = inttoptr i64 %v2 to ptr
  %v24 = tail call { float, float } asm "call ($0, $1), _optix_get_triangle_barycentrics, ();", "=f,=f"() #2
  %v25 = extractvalue { float, float } %v24, 0
  %v26 = extractvalue { float, float } %v24, 1
  %v27 = tail call i64 asm "call ($0), _optix_call_direct_callable,($1);", "=l,r"(i32 0) #2
  %v28 = tail call float asm sideeffect "{\0A\09.param .b32 nvrs_rv;\0A\09.param .b32 nvrs_a0;\0A\09st.param.b32 [nvrs_a0], $2;\0A\09nvrs_proto: .callprototype (.param .b32 _) _ (.param .b32 _);\0A\09call (nvrs_rv), $1, (nvrs_a0), nvrs_proto;\0A\09ld.param.b32 $0, [nvrs_rv];\0A}", "=f,l,f"(i64 %v27, float %v25) #2
  %v3 = tail call i64 asm "call ($0), _optix_call_direct_callable,($1);", "=l,r"(i32 0) #2
  %v29 = tail call float asm sideeffect "{\0A\09.param .b32 nvrs_rv;\0A\09.param .b32 nvrs_a0;\0A\09st.param.b32 [nvrs_a0], $2;\0A\09nvrs_proto: .callprototype (.param .b32 _) _ (.param .b32 _);\0A\09call (nvrs_rv), $1, (nvrs_a0), nvrs_proto;\0A\09ld.param.b32 $0, [nvrs_rv];\0A}", "=f,l,f"(i64 %v3, float %v26) #2
  %v6 = load float, ptr %v23, align 4
  %v7 = fmul contract float %v28, %v6
  %v9 = getelementptr inbounds nuw i8, ptr %v23, i64 4
  %v10 = load float, ptr %v9, align 4
  %v11 = fmul contract float %v29, %v10
  %v12 = fmul contract float %v11, 5.000000e-01
  %v14 = getelementptr inbounds nuw i8, ptr %v23, i64 8
  %v15 = load float, ptr %v14, align 4
  %v16 = fsub contract float 1.000000e+00, %v25
  %v17 = fsub contract float %v16, %v26
  %v18 = fmul contract float %v17, %v15
  %v22 = tail call i32 asm "call ($0), _optix_read_primitive_idx, ();", "=r"() #2
  %0 = bitcast float %v7 to i32
  %1 = bitcast float %v12 to i32
  %2 = bitcast float %v18 to i32
  tail call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 0, i32 %0) #2
  tail call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 1, i32 %1) #2
  tail call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 2, i32 %2) #2
  tail call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 3, i32 %v22) #2
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none)
define float @__direct_callable__double(float %v0) #1 {
entry:
  %v2 = fmul contract float %v0, 2.000000e+00
  ret float %v2
}

attributes #0 = { convergent nounwind "target-cpu"="sm_86" }
attributes #1 = { mustprogress nofree norecurse nosync nounwind willreturn memory(none) "target-cpu"="sm_86" }
attributes #2 = { convergent nounwind }
