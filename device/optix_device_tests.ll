; ModuleID = 'builtin.module'
source_filename = "optix_device_tests"
target datalayout = "e-p:64:64:64-p3:32:32:32-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64-i128:128:128-f32:32:32-f64:64:64-f128:128:128-v16:16:16-v32:32:32-v64:64:64-v128:128:128-n16:32:64-a:8:8"
target triple = "nvptx64-nvidia-cuda"

@params = addrspace(4) externally_initialized global [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", align 8
define ptx_kernel void @__raygen__render() #0 {
entry:
  %__cuda_oxide_local_x32093132096f726967696e095b6633323b20335d_ = alloca [3 x float], align 4
  %__cuda_oxide_local_x31310933360972617909526179_ = alloca { [3 x float], [3 x float], float, float, float }, align 4
  %v2 = alloca [3 x float], align 4
  %__cuda_oxide_local_x3133093136097368616465095368616465_ = alloca { [3 x float], i32 }, align 4
  %v4 = alloca [3 x float], align 4
  %v5 = alloca { i32, i32, i32, i32, i32, i32 }, align 4
  br label %bb0
bb0:
  %v6 = call i32 asm "call ($0), _optix_get_launch_index_x, ();", "=r"() #0
  br label %bb2
bb1:
  %v7 = inttoptr i64 %v35 to ptr
  %v8 = mul i32 %v28, %v39
  %v9 = add i32 %v8, %v6
  %v10 = zext i32 %v9 to i64
  %v11 = getelementptr inbounds [4 x float], ptr %v7, i64 %v10
  %v12 = getelementptr inbounds { [3 x float], i32 }, ptr %__cuda_oxide_local_x3133093136097368616465095368616465_, i32 0, i32 0
  %v13 = getelementptr inbounds float, ptr %v12, i64 0
  %v14 = load float, ptr %v13, align 4
  %v15 = getelementptr inbounds { [3 x float], i32 }, ptr %__cuda_oxide_local_x3133093136097368616465095368616465_, i32 0, i32 0
  %v16 = getelementptr inbounds float, ptr %v15, i64 1
  %v17 = load float, ptr %v16, align 4
  %v18 = getelementptr inbounds { [3 x float], i32 }, ptr %__cuda_oxide_local_x3133093136097368616465095368616465_, i32 0, i32 0
  %v19 = getelementptr inbounds float, ptr %v18, i64 2
  %v20 = load float, ptr %v19, align 4
  %v21 = getelementptr inbounds { [3 x float], i32 }, ptr %__cuda_oxide_local_x3133093136097368616465095368616465_, i32 0, i32 1
  %v22 = load i32, ptr %v21, align 4
  %v23 = sitofp i32 %v22 to float
  %v24 = getelementptr inbounds float, ptr %v11, i64 0
  store float %v14, ptr %v24, align 4
  %v25 = getelementptr inbounds float, ptr %v11, i64 1
  store float %v17, ptr %v25, align 4
  %v26 = getelementptr inbounds float, ptr %v11, i64 2
  store float %v20, ptr %v26, align 4
  %v27 = getelementptr inbounds float, ptr %v11, i64 3
  store float %v23, ptr %v27, align 4
  ret void
bb2:
  %v28 = call i32 asm "call ($0), _optix_get_launch_index_y, ();", "=r"() #0
  br label %bb3
bb3:
  %v29 = call i32 asm "call ($0), _optix_get_launch_index_z, ();", "=r"() #0
  br label %bb4
bb4:
  %v30 = addrspacecast ptr addrspace(4) @params to ptr
  %v31 = addrspacecast ptr %v30 to ptr addrspace(4)
  %v32 = getelementptr inbounds { { { i64, i64, i32, i32 } } }, ptr addrspace(4) %v31, i32 0, i32 0
  %v33 = addrspacecast ptr addrspace(4) %v32 to ptr
  %v34 = getelementptr inbounds { i64, i64, i32, i32 }, ptr %v33, i32 0, i32 0
  %v35 = load i64, ptr %v34, align 8
  %v36 = getelementptr inbounds { i64, i64, i32, i32 }, ptr %v33, i32 0, i32 1
  %v37 = load i64, ptr %v36, align 8
  %v38 = getelementptr inbounds { i64, i64, i32, i32 }, ptr %v33, i32 0, i32 2
  %v39 = load i32, ptr %v38, align 8
  %v40 = getelementptr inbounds { i64, i64, i32, i32 }, ptr %v33, i32 0, i32 3
  %v41 = load i32, ptr %v40, align 4
  %v42 = uitofp i32 %v6 to float
  %v43 = fadd contract float %v42, 0.5
  %v44 = uitofp i32 %v39 to float
  %v45 = fdiv contract float %v43, %v44
  %v46 = uitofp i32 %v28 to float
  %v47 = fadd contract float %v46, 0.5
  %v48 = uitofp i32 %v41 to float
  %v49 = fdiv contract float %v47, %v48
  %v50 = insertvalue [3 x float] undef, float %v45, 0
  %v51 = insertvalue [3 x float] %v50, float %v49, 1
  %v52 = insertvalue [3 x float] %v51, float -1.0, 2
  %v53 = insertvalue [3 x float] undef, float 0.0, 0
  %v54 = insertvalue [3 x float] %v53, float 0.0, 1
  %v55 = insertvalue [3 x float] %v54, float 1.0, 2
  %v56 = insertvalue { [3 x float], [3 x float], float, float, float } undef, [3 x float] %v52, 0
  %v57 = insertvalue { [3 x float], [3 x float], float, float, float } %v56, [3 x float] %v55, 1
  %v58 = insertvalue { [3 x float], [3 x float], float, float, float } %v57, float 0.0, 2
  %v59 = insertvalue { [3 x float], [3 x float], float, float, float } %v58, float 10000000272564224.0, 3
  %v60 = insertvalue { [3 x float], [3 x float], float, float, float } %v59, float 0.0, 4
  store { [3 x float], [3 x float], float, float, float } %v60, ptr %__cuda_oxide_local_x31310933360972617909526179_, align 4
  %v61 = insertvalue [3 x float] undef, float 0.0, 0
  %v62 = insertvalue [3 x float] %v61, float 0.0, 1
  %v63 = insertvalue [3 x float] %v62, float 0.0, 2
  %v64 = insertvalue { [3 x float], i32 } undef, [3 x float] %v63, 0
  %v65 = insertvalue { [3 x float], i32 } %v64, i32 4294967294, 1
  store { [3 x float], i32 } %v65, ptr %__cuda_oxide_local_x3133093136097368616465095368616465_, align 4
  %v66 = insertvalue { i32, i32, i32, i32, i32, i32 } undef, i32 255, 0
  %v67 = insertvalue { i32, i32, i32, i32, i32, i32 } %v66, i32 0, 1
  %v68 = insertvalue { i32, i32, i32, i32, i32, i32 } %v67, i32 0, 2
  %v69 = insertvalue { i32, i32, i32, i32, i32, i32 } %v68, i32 1, 3
  %v70 = insertvalue { i32, i32, i32, i32, i32, i32 } %v69, i32 0, 4
  %v71 = insertvalue { i32, i32, i32, i32, i32, i32 } %v70, i32 0, 5
  store { i32, i32, i32, i32, i32, i32 } %v71, ptr %v5, align 4
  call void @_RINvNtCs2WD1vkMjalt_12optix_device5trace5traceNtCs85rhR4mBFTl_18optix_device_tests5ShadeEBK_(i64 %v37, ptr %__cuda_oxide_local_x31310933360972617909526179_, ptr %v5, ptr %__cuda_oxide_local_x3133093136097368616465095368616465_) #0
  br label %bb1
}

define ptx_kernel void @__miss__background() #0 {
entry:
  %v0 = alloca { [3 x float], i32 }, align 4
  br label %bb0
bb0:
  %v1 = call i64 asm "call ($0), _optix_get_sbt_data_ptr_64, ();", "=l"() #0
  br label %bb2
bb1:
  ret void
bb2:
  %v2 = inttoptr i64 %v1 to ptr
  %v3 = getelementptr inbounds { [3 x float], float }, ptr %v2, i32 0, i32 0
  %v4 = load [3 x float], ptr %v3, align 4
  %v5 = insertvalue { [3 x float], i32 } undef, [3 x float] %v4, 0
  %v6 = insertvalue { [3 x float], i32 } %v5, i32 4294967295, 1
  store { [3 x float], i32 } %v6, ptr %v0, align 4
  call void @_RINvNtCs2WD1vkMjalt_12optix_device7payload3setNtCs85rhR4mBFTl_18optix_device_tests5ShadeEBK_(ptr %v0) #0
  br label %bb1
}

define ptx_kernel void @__closesthit__shade() #0 {
entry:
  %__cuda_oxide_local_x3509313209726762095b6633323b20335d_ = alloca [3 x float], align 4
  %v1 = alloca { [3 x float], i32 }, align 4
  br label %bb0
bb0:
  %v2 = call i64 asm "call ($0), _optix_get_sbt_data_ptr_64, ();", "=l"() #0
  br label %bb4
bb1:
  %v3 = call i64 asm "call ($0), _optix_call_direct_callable,($1);", "=l,r"(i32 0) #0
  br label %bb7
bb2:
  %v4 = getelementptr inbounds { [3 x float], float }, ptr %v23, i32 0, i32 0
  %v5 = getelementptr inbounds float, ptr %v4, i64 0
  %v6 = load float, ptr %v5, align 4
  %v7 = fmul contract float %v6, %v28
  %v8 = getelementptr inbounds { [3 x float], float }, ptr %v23, i32 0, i32 0
  %v9 = getelementptr inbounds float, ptr %v8, i64 1
  %v10 = load float, ptr %v9, align 4
  %v11 = fmul contract float %v10, %v29
  %v12 = fmul contract float %v11, 0.5
  %v13 = getelementptr inbounds { [3 x float], float }, ptr %v23, i32 0, i32 0
  %v14 = getelementptr inbounds float, ptr %v13, i64 2
  %v15 = load float, ptr %v14, align 4
  %v16 = fsub contract float 1.0, %v25
  %v17 = fsub contract float %v16, %v26
  %v18 = fmul contract float %v15, %v17
  %v19 = insertvalue [3 x float] undef, float %v7, 0
  %v20 = insertvalue [3 x float] %v19, float %v12, 1
  %v21 = insertvalue [3 x float] %v20, float %v18, 2
  %v22 = call i32 asm "call ($0), _optix_read_primitive_idx, ();", "=r"() #0
  br label %bb8
bb3:
  ret void
bb4:
  %v23 = inttoptr i64 %v2 to ptr
  %v24 = call { float, float } asm "call ($0, $1), _optix_get_triangle_barycentrics, ();", "=f,=f"() #0
  %v25 = extractvalue { float, float } %v24, 0
  %v26 = extractvalue { float, float } %v24, 1
  br label %bb5
bb5:
  %v27 = call i64 asm "call ($0), _optix_call_direct_callable,($1);", "=l,r"(i32 0) #0
  br label %bb6
bb6:
  %v28 = call float asm sideeffect "{\0A\09.param .b32 nvrs_rv;\0A\09.param .b32 nvrs_a0;\0A\09st.param.b32 [nvrs_a0], $2;\0A\09nvrs_proto: .callprototype (.param .b32 _) _ (.param .b32 _);\0A\09call (nvrs_rv), $1, (nvrs_a0), nvrs_proto;\0A\09ld.param.b32 $0, [nvrs_rv];\0A}", "=f,l,f"(i64 %v27, float %v25) #0
  br label %bb1
bb7:
  %v29 = call float asm sideeffect "{\0A\09.param .b32 nvrs_rv;\0A\09.param .b32 nvrs_a0;\0A\09st.param.b32 [nvrs_a0], $2;\0A\09nvrs_proto: .callprototype (.param .b32 _) _ (.param .b32 _);\0A\09call (nvrs_rv), $1, (nvrs_a0), nvrs_proto;\0A\09ld.param.b32 $0, [nvrs_rv];\0A}", "=f,l,f"(i64 %v3, float %v26) #0
  br label %bb2
bb8:
  %v30 = bitcast i32 %v22 to i32
  %v31 = insertvalue { [3 x float], i32 } undef, [3 x float] %v21, 0
  %v32 = insertvalue { [3 x float], i32 } %v31, i32 %v30, 1
  store { [3 x float], i32 } %v32, ptr %v1, align 4
  call void @_RINvNtCs2WD1vkMjalt_12optix_device7payload3setNtCs85rhR4mBFTl_18optix_device_tests5ShadeEBK_(ptr %v1) #0
  br label %bb3
}

define float @__direct_callable__double(float %v0) #0 {
entry:
  br label %bb0
bb0:
  %v1 = phi float [ %v0, %entry ]
  %v2 = fmul contract float 2.0, %v1
  ret float %v2
}

declare void @llvm.memcpy.p0.p0.i64(ptr, ptr, i64, i1)

define void @_RINvNtCs2WD1vkMjalt_12optix_device5trace5traceNtCs85rhR4mBFTl_18optix_device_tests5ShadeEBK_(i64 %v0, ptr %v1, ptr %v2, ptr %v3) alwaysinline #0 {
entry:
  %__cuda_oxide_local_x35093132380970095b7533323b2033325d_ = alloca [32 x i32], align 4
  %__cuda_oxide_local_x3132093132380972095b7533323b2033325d_ = alloca [32 x i32], align 4
  %v6 = alloca [32 x i32], align 4
  %v7 = alloca { [0 x i32], { { { [3 x float], i32 } } } }, align 4
  %v8 = alloca { [0 x i32], { { { [3 x float], i32 } } } }, align 4
  br label %bb0
bb0:
  %v9 = phi i64 [ %v0, %entry ]
  %v10 = phi ptr [ %v1, %entry ]
  %v11 = phi ptr [ %v2, %entry ]
  %v12 = phi ptr [ %v3, %entry ]
  %v13 = getelementptr inbounds i32, ptr %v6, i64 0
  store i32 0, ptr %v13, align 4
  %v14 = getelementptr inbounds i32, ptr %v6, i64 1
  store i32 0, ptr %v14, align 4
  %v15 = getelementptr inbounds i32, ptr %v6, i64 2
  store i32 0, ptr %v15, align 4
  %v16 = getelementptr inbounds i32, ptr %v6, i64 3
  store i32 0, ptr %v16, align 4
  %v17 = getelementptr inbounds i32, ptr %v6, i64 4
  store i32 0, ptr %v17, align 4
  %v18 = getelementptr inbounds i32, ptr %v6, i64 5
  store i32 0, ptr %v18, align 4
  %v19 = getelementptr inbounds i32, ptr %v6, i64 6
  store i32 0, ptr %v19, align 4
  %v20 = getelementptr inbounds i32, ptr %v6, i64 7
  store i32 0, ptr %v20, align 4
  %v21 = getelementptr inbounds i32, ptr %v6, i64 8
  store i32 0, ptr %v21, align 4
  %v22 = getelementptr inbounds i32, ptr %v6, i64 9
  store i32 0, ptr %v22, align 4
  %v23 = getelementptr inbounds i32, ptr %v6, i64 10
  store i32 0, ptr %v23, align 4
  %v24 = getelementptr inbounds i32, ptr %v6, i64 11
  store i32 0, ptr %v24, align 4
  %v25 = getelementptr inbounds i32, ptr %v6, i64 12
  store i32 0, ptr %v25, align 4
  %v26 = getelementptr inbounds i32, ptr %v6, i64 13
  store i32 0, ptr %v26, align 4
  %v27 = getelementptr inbounds i32, ptr %v6, i64 14
  store i32 0, ptr %v27, align 4
  %v28 = getelementptr inbounds i32, ptr %v6, i64 15
  store i32 0, ptr %v28, align 4
  %v29 = getelementptr inbounds i32, ptr %v6, i64 16
  store i32 0, ptr %v29, align 4
  %v30 = getelementptr inbounds i32, ptr %v6, i64 17
  store i32 0, ptr %v30, align 4
  %v31 = getelementptr inbounds i32, ptr %v6, i64 18
  store i32 0, ptr %v31, align 4
  %v32 = getelementptr inbounds i32, ptr %v6, i64 19
  store i32 0, ptr %v32, align 4
  %v33 = getelementptr inbounds i32, ptr %v6, i64 20
  store i32 0, ptr %v33, align 4
  %v34 = getelementptr inbounds i32, ptr %v6, i64 21
  store i32 0, ptr %v34, align 4
  %v35 = getelementptr inbounds i32, ptr %v6, i64 22
  store i32 0, ptr %v35, align 4
  %v36 = getelementptr inbounds i32, ptr %v6, i64 23
  store i32 0, ptr %v36, align 4
  %v37 = getelementptr inbounds i32, ptr %v6, i64 24
  store i32 0, ptr %v37, align 4
  %v38 = getelementptr inbounds i32, ptr %v6, i64 25
  store i32 0, ptr %v38, align 4
  %v39 = getelementptr inbounds i32, ptr %v6, i64 26
  store i32 0, ptr %v39, align 4
  %v40 = getelementptr inbounds i32, ptr %v6, i64 27
  store i32 0, ptr %v40, align 4
  %v41 = getelementptr inbounds i32, ptr %v6, i64 28
  store i32 0, ptr %v41, align 4
  %v42 = getelementptr inbounds i32, ptr %v6, i64 29
  store i32 0, ptr %v42, align 4
  %v43 = getelementptr inbounds i32, ptr %v6, i64 30
  store i32 0, ptr %v43, align 4
  %v44 = getelementptr inbounds i32, ptr %v6, i64 31
  store i32 0, ptr %v44, align 4
  %v45 = insertvalue { ptr, i64 } undef, ptr %v6, 0
  %v46 = insertvalue { ptr, i64 } %v45, i64 32, 1
  %v47 = extractvalue { ptr, i64 } %v46, 0
  call void @llvm.memcpy.p0.p0.i64(ptr %v47, ptr %v12, i64 16, i1 0) #0
  %v49 = load [32 x i32], ptr %v6, align 4
  store [32 x i32] %v49, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, align 4
  %v50 = getelementptr inbounds { [3 x float], [3 x float], float, float, float }, ptr %v10, i32 0, i32 0
  %v51 = getelementptr inbounds float, ptr %v50, i64 0
  %v52 = load float, ptr %v51, align 4
  %v53 = getelementptr inbounds { [3 x float], [3 x float], float, float, float }, ptr %v10, i32 0, i32 0
  %v54 = getelementptr inbounds float, ptr %v53, i64 1
  %v55 = load float, ptr %v54, align 4
  %v56 = getelementptr inbounds { [3 x float], [3 x float], float, float, float }, ptr %v10, i32 0, i32 0
  %v57 = getelementptr inbounds float, ptr %v56, i64 2
  %v58 = load float, ptr %v57, align 4
  %v59 = getelementptr inbounds { [3 x float], [3 x float], float, float, float }, ptr %v10, i32 0, i32 1
  %v60 = getelementptr inbounds float, ptr %v59, i64 0
  %v61 = load float, ptr %v60, align 4
  %v62 = getelementptr inbounds { [3 x float], [3 x float], float, float, float }, ptr %v10, i32 0, i32 1
  %v63 = getelementptr inbounds float, ptr %v62, i64 1
  %v64 = load float, ptr %v63, align 4
  %v65 = getelementptr inbounds { [3 x float], [3 x float], float, float, float }, ptr %v10, i32 0, i32 1
  %v66 = getelementptr inbounds float, ptr %v65, i64 2
  %v67 = load float, ptr %v66, align 4
  %v68 = getelementptr inbounds { i32, i32, i32, i32, i32, i32 }, ptr %v11, i32 0, i32 5
  %v69 = load i32, ptr %v68, align 4
  %v70 = getelementptr inbounds { [3 x float], [3 x float], float, float, float }, ptr %v10, i32 0, i32 2
  %v71 = load float, ptr %v70, align 4
  %v72 = getelementptr inbounds { [3 x float], [3 x float], float, float, float }, ptr %v10, i32 0, i32 3
  %v73 = load float, ptr %v72, align 4
  %v74 = getelementptr inbounds { [3 x float], [3 x float], float, float, float }, ptr %v10, i32 0, i32 4
  %v75 = load float, ptr %v74, align 4
  %v76 = getelementptr inbounds { i32, i32, i32, i32, i32, i32 }, ptr %v11, i32 0, i32 0
  %v77 = load i32, ptr %v76, align 4
  %v78 = getelementptr inbounds { i32, i32, i32, i32, i32, i32 }, ptr %v11, i32 0, i32 1
  %v79 = load i32, ptr %v78, align 4
  %v80 = getelementptr inbounds { i32, i32, i32, i32, i32, i32 }, ptr %v11, i32 0, i32 2
  %v81 = load i32, ptr %v80, align 4
  %v82 = getelementptr inbounds { i32, i32, i32, i32, i32, i32 }, ptr %v11, i32 0, i32 3
  %v83 = load i32, ptr %v82, align 4
  %v84 = getelementptr inbounds { i32, i32, i32, i32, i32, i32 }, ptr %v11, i32 0, i32 4
  %v85 = load i32, ptr %v84, align 4
  %v86 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 0
  %v87 = load i32, ptr %v86, align 4
  %v88 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 1
  %v89 = load i32, ptr %v88, align 4
  %v90 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 2
  %v91 = load i32, ptr %v90, align 4
  %v92 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 3
  %v93 = load i32, ptr %v92, align 4
  %v94 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 4
  %v95 = load i32, ptr %v94, align 4
  %v96 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 5
  %v97 = load i32, ptr %v96, align 4
  %v98 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 6
  %v99 = load i32, ptr %v98, align 4
  %v100 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 7
  %v101 = load i32, ptr %v100, align 4
  %v102 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 8
  %v103 = load i32, ptr %v102, align 4
  %v104 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 9
  %v105 = load i32, ptr %v104, align 4
  %v106 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 10
  %v107 = load i32, ptr %v106, align 4
  %v108 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 11
  %v109 = load i32, ptr %v108, align 4
  %v110 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 12
  %v111 = load i32, ptr %v110, align 4
  %v112 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 13
  %v113 = load i32, ptr %v112, align 4
  %v114 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 14
  %v115 = load i32, ptr %v114, align 4
  %v116 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 15
  %v117 = load i32, ptr %v116, align 4
  %v118 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 16
  %v119 = load i32, ptr %v118, align 4
  %v120 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 17
  %v121 = load i32, ptr %v120, align 4
  %v122 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 18
  %v123 = load i32, ptr %v122, align 4
  %v124 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 19
  %v125 = load i32, ptr %v124, align 4
  %v126 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 20
  %v127 = load i32, ptr %v126, align 4
  %v128 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 21
  %v129 = load i32, ptr %v128, align 4
  %v130 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 22
  %v131 = load i32, ptr %v130, align 4
  %v132 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 23
  %v133 = load i32, ptr %v132, align 4
  %v134 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 24
  %v135 = load i32, ptr %v134, align 4
  %v136 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 25
  %v137 = load i32, ptr %v136, align 4
  %v138 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 26
  %v139 = load i32, ptr %v138, align 4
  %v140 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 27
  %v141 = load i32, ptr %v140, align 4
  %v142 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 28
  %v143 = load i32, ptr %v142, align 4
  %v144 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 29
  %v145 = load i32, ptr %v144, align 4
  %v146 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 30
  %v147 = load i32, ptr %v146, align 4
  %v148 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x35093132380970095b7533323b2033325d_, i64 31
  %v149 = load i32, ptr %v148, align 4
  %v150 = call [32 x i32] @optix_device__raw__trace_typed_32(i32 %v69, i64 %v9, float %v52, float %v55, float %v58, float %v61, float %v64, float %v67, float %v71, float %v73, float %v75, i32 %v77, i32 %v79, i32 %v81, i32 %v83, i32 %v85, i32 4, i32 %v87, i32 %v89, i32 %v91, i32 %v93, i32 %v95, i32 %v97, i32 %v99, i32 %v101, i32 %v103, i32 %v105, i32 %v107, i32 %v109, i32 %v111, i32 %v113, i32 %v115, i32 %v117, i32 %v119, i32 %v121, i32 %v123, i32 %v125, i32 %v127, i32 %v129, i32 %v131, i32 %v133, i32 %v135, i32 %v137, i32 %v139, i32 %v141, i32 %v143, i32 %v145, i32 %v147, i32 %v149) #0
  store [32 x i32] %v150, ptr %__cuda_oxide_local_x3132093132380972095b7533323b2033325d_, align 4
  br label %bb1
bb1:
  store { [0 x i32], { { { [3 x float], i32 } } } } undef, ptr %v7, align 4
  %v151 = insertvalue { ptr, i64 } undef, ptr %__cuda_oxide_local_x3132093132380972095b7533323b2033325d_, 0
  %v152 = insertvalue { ptr, i64 } %v151, i64 32, 1
  %v153 = extractvalue { ptr, i64 } %v152, 0
  call void @llvm.memcpy.p0.p0.i64(ptr %v7, ptr %v153, i64 16, i1 0) #0
  %v155 = load { [0 x i32], { { { [3 x float], i32 } } } }, ptr %v7, align 4
  br label %bb2
bb2:
  store { [0 x i32], { { { [3 x float], i32 } } } } %v155, ptr %v8, align 4
  %v156 = load { [3 x float], i32 }, ptr %v8, align 4
  store { [3 x float], i32 } %v156, ptr %v12, align 4
  ret void
}

define void @_RINvNtCs2WD1vkMjalt_12optix_device7payload3setNtCs85rhR4mBFTl_18optix_device_tests5ShadeEBK_(ptr %v0) alwaysinline #0 {
entry:
  %__cuda_oxide_local_x32093132380977095b7533323b2033325d_ = alloca [32 x i32], align 4
  %v2 = alloca [32 x i32], align 4
  br label %bb0
bb0:
  %v3 = phi ptr [ %v0, %entry ]
  %v4 = getelementptr inbounds i32, ptr %v2, i64 0
  store i32 0, ptr %v4, align 4
  %v5 = getelementptr inbounds i32, ptr %v2, i64 1
  store i32 0, ptr %v5, align 4
  %v6 = getelementptr inbounds i32, ptr %v2, i64 2
  store i32 0, ptr %v6, align 4
  %v7 = getelementptr inbounds i32, ptr %v2, i64 3
  store i32 0, ptr %v7, align 4
  %v8 = getelementptr inbounds i32, ptr %v2, i64 4
  store i32 0, ptr %v8, align 4
  %v9 = getelementptr inbounds i32, ptr %v2, i64 5
  store i32 0, ptr %v9, align 4
  %v10 = getelementptr inbounds i32, ptr %v2, i64 6
  store i32 0, ptr %v10, align 4
  %v11 = getelementptr inbounds i32, ptr %v2, i64 7
  store i32 0, ptr %v11, align 4
  %v12 = getelementptr inbounds i32, ptr %v2, i64 8
  store i32 0, ptr %v12, align 4
  %v13 = getelementptr inbounds i32, ptr %v2, i64 9
  store i32 0, ptr %v13, align 4
  %v14 = getelementptr inbounds i32, ptr %v2, i64 10
  store i32 0, ptr %v14, align 4
  %v15 = getelementptr inbounds i32, ptr %v2, i64 11
  store i32 0, ptr %v15, align 4
  %v16 = getelementptr inbounds i32, ptr %v2, i64 12
  store i32 0, ptr %v16, align 4
  %v17 = getelementptr inbounds i32, ptr %v2, i64 13
  store i32 0, ptr %v17, align 4
  %v18 = getelementptr inbounds i32, ptr %v2, i64 14
  store i32 0, ptr %v18, align 4
  %v19 = getelementptr inbounds i32, ptr %v2, i64 15
  store i32 0, ptr %v19, align 4
  %v20 = getelementptr inbounds i32, ptr %v2, i64 16
  store i32 0, ptr %v20, align 4
  %v21 = getelementptr inbounds i32, ptr %v2, i64 17
  store i32 0, ptr %v21, align 4
  %v22 = getelementptr inbounds i32, ptr %v2, i64 18
  store i32 0, ptr %v22, align 4
  %v23 = getelementptr inbounds i32, ptr %v2, i64 19
  store i32 0, ptr %v23, align 4
  %v24 = getelementptr inbounds i32, ptr %v2, i64 20
  store i32 0, ptr %v24, align 4
  %v25 = getelementptr inbounds i32, ptr %v2, i64 21
  store i32 0, ptr %v25, align 4
  %v26 = getelementptr inbounds i32, ptr %v2, i64 22
  store i32 0, ptr %v26, align 4
  %v27 = getelementptr inbounds i32, ptr %v2, i64 23
  store i32 0, ptr %v27, align 4
  %v28 = getelementptr inbounds i32, ptr %v2, i64 24
  store i32 0, ptr %v28, align 4
  %v29 = getelementptr inbounds i32, ptr %v2, i64 25
  store i32 0, ptr %v29, align 4
  %v30 = getelementptr inbounds i32, ptr %v2, i64 26
  store i32 0, ptr %v30, align 4
  %v31 = getelementptr inbounds i32, ptr %v2, i64 27
  store i32 0, ptr %v31, align 4
  %v32 = getelementptr inbounds i32, ptr %v2, i64 28
  store i32 0, ptr %v32, align 4
  %v33 = getelementptr inbounds i32, ptr %v2, i64 29
  store i32 0, ptr %v33, align 4
  %v34 = getelementptr inbounds i32, ptr %v2, i64 30
  store i32 0, ptr %v34, align 4
  %v35 = getelementptr inbounds i32, ptr %v2, i64 31
  store i32 0, ptr %v35, align 4
  %v36 = insertvalue { ptr, i64 } undef, ptr %v2, 0
  %v37 = insertvalue { ptr, i64 } %v36, i64 32, 1
  %v38 = extractvalue { ptr, i64 } %v37, 0
  call void @llvm.memcpy.p0.p0.i64(ptr %v38, ptr %v3, i64 16, i1 0) #0
  %v40 = load [32 x i32], ptr %v2, align 4
  store [32 x i32] %v40, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, align 4
  %v41 = icmp ult i32 0, 4
  %v42 = xor i1 %v41, 1
  br i1 %v42, label %bb2, label %bb1
bb1:
  %v43 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 0
  %v44 = load i32, ptr %v43, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 0, i32 %v44) #0
  br label %bb65
bb2:
  %v46 = icmp ult i32 1, 4
  %v47 = xor i1 %v46, 1
  br i1 %v47, label %bb4, label %bb3
bb3:
  %v48 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 1
  %v49 = load i32, ptr %v48, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 1, i32 %v49) #0
  br label %bb66
bb4:
  %v51 = icmp ult i32 2, 4
  %v52 = xor i1 %v51, 1
  br i1 %v52, label %bb6, label %bb5
bb5:
  %v53 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 2
  %v54 = load i32, ptr %v53, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 2, i32 %v54) #0
  br label %bb67
bb6:
  %v56 = icmp ult i32 3, 4
  %v57 = xor i1 %v56, 1
  br i1 %v57, label %bb8, label %bb7
bb7:
  %v58 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 3
  %v59 = load i32, ptr %v58, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 3, i32 %v59) #0
  br label %bb68
bb8:
  %v61 = icmp ult i32 4, 4
  %v62 = xor i1 %v61, 1
  br i1 %v62, label %bb10, label %bb9
bb9:
  %v63 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 4
  %v64 = load i32, ptr %v63, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 4, i32 %v64) #0
  br label %bb69
bb10:
  %v66 = icmp ult i32 5, 4
  %v67 = xor i1 %v66, 1
  br i1 %v67, label %bb12, label %bb11
bb11:
  %v68 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 5
  %v69 = load i32, ptr %v68, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 5, i32 %v69) #0
  br label %bb70
bb12:
  %v71 = icmp ult i32 6, 4
  %v72 = xor i1 %v71, 1
  br i1 %v72, label %bb14, label %bb13
bb13:
  %v73 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 6
  %v74 = load i32, ptr %v73, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 6, i32 %v74) #0
  br label %bb71
bb14:
  %v76 = icmp ult i32 7, 4
  %v77 = xor i1 %v76, 1
  br i1 %v77, label %bb16, label %bb15
bb15:
  %v78 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 7
  %v79 = load i32, ptr %v78, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 7, i32 %v79) #0
  br label %bb72
bb16:
  %v81 = icmp ult i32 8, 4
  %v82 = xor i1 %v81, 1
  br i1 %v82, label %bb18, label %bb17
bb17:
  %v83 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 8
  %v84 = load i32, ptr %v83, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 8, i32 %v84) #0
  br label %bb73
bb18:
  %v86 = icmp ult i32 9, 4
  %v87 = xor i1 %v86, 1
  br i1 %v87, label %bb20, label %bb19
bb19:
  %v88 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 9
  %v89 = load i32, ptr %v88, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 9, i32 %v89) #0
  br label %bb74
bb20:
  %v91 = icmp ult i32 10, 4
  %v92 = xor i1 %v91, 1
  br i1 %v92, label %bb22, label %bb21
bb21:
  %v93 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 10
  %v94 = load i32, ptr %v93, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 10, i32 %v94) #0
  br label %bb75
bb22:
  %v96 = icmp ult i32 11, 4
  %v97 = xor i1 %v96, 1
  br i1 %v97, label %bb24, label %bb23
bb23:
  %v98 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 11
  %v99 = load i32, ptr %v98, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 11, i32 %v99) #0
  br label %bb76
bb24:
  %v101 = icmp ult i32 12, 4
  %v102 = xor i1 %v101, 1
  br i1 %v102, label %bb26, label %bb25
bb25:
  %v103 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 12
  %v104 = load i32, ptr %v103, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 12, i32 %v104) #0
  br label %bb77
bb26:
  %v106 = icmp ult i32 13, 4
  %v107 = xor i1 %v106, 1
  br i1 %v107, label %bb28, label %bb27
bb27:
  %v108 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 13
  %v109 = load i32, ptr %v108, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 13, i32 %v109) #0
  br label %bb78
bb28:
  %v111 = icmp ult i32 14, 4
  %v112 = xor i1 %v111, 1
  br i1 %v112, label %bb30, label %bb29
bb29:
  %v113 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 14
  %v114 = load i32, ptr %v113, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 14, i32 %v114) #0
  br label %bb79
bb30:
  %v116 = icmp ult i32 15, 4
  %v117 = xor i1 %v116, 1
  br i1 %v117, label %bb32, label %bb31
bb31:
  %v118 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 15
  %v119 = load i32, ptr %v118, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 15, i32 %v119) #0
  br label %bb80
bb32:
  %v121 = icmp ult i32 16, 4
  %v122 = xor i1 %v121, 1
  br i1 %v122, label %bb34, label %bb33
bb33:
  %v123 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 16
  %v124 = load i32, ptr %v123, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 16, i32 %v124) #0
  br label %bb81
bb34:
  %v126 = icmp ult i32 17, 4
  %v127 = xor i1 %v126, 1
  br i1 %v127, label %bb36, label %bb35
bb35:
  %v128 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 17
  %v129 = load i32, ptr %v128, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 17, i32 %v129) #0
  br label %bb82
bb36:
  %v131 = icmp ult i32 18, 4
  %v132 = xor i1 %v131, 1
  br i1 %v132, label %bb38, label %bb37
bb37:
  %v133 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 18
  %v134 = load i32, ptr %v133, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 18, i32 %v134) #0
  br label %bb83
bb38:
  %v136 = icmp ult i32 19, 4
  %v137 = xor i1 %v136, 1
  br i1 %v137, label %bb40, label %bb39
bb39:
  %v138 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 19
  %v139 = load i32, ptr %v138, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 19, i32 %v139) #0
  br label %bb84
bb40:
  %v141 = icmp ult i32 20, 4
  %v142 = xor i1 %v141, 1
  br i1 %v142, label %bb42, label %bb41
bb41:
  %v143 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 20
  %v144 = load i32, ptr %v143, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 20, i32 %v144) #0
  br label %bb85
bb42:
  %v146 = icmp ult i32 21, 4
  %v147 = xor i1 %v146, 1
  br i1 %v147, label %bb44, label %bb43
bb43:
  %v148 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 21
  %v149 = load i32, ptr %v148, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 21, i32 %v149) #0
  br label %bb86
bb44:
  %v151 = icmp ult i32 22, 4
  %v152 = xor i1 %v151, 1
  br i1 %v152, label %bb46, label %bb45
bb45:
  %v153 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 22
  %v154 = load i32, ptr %v153, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 22, i32 %v154) #0
  br label %bb87
bb46:
  %v156 = icmp ult i32 23, 4
  %v157 = xor i1 %v156, 1
  br i1 %v157, label %bb48, label %bb47
bb47:
  %v158 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 23
  %v159 = load i32, ptr %v158, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 23, i32 %v159) #0
  br label %bb88
bb48:
  %v161 = icmp ult i32 24, 4
  %v162 = xor i1 %v161, 1
  br i1 %v162, label %bb50, label %bb49
bb49:
  %v163 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 24
  %v164 = load i32, ptr %v163, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 24, i32 %v164) #0
  br label %bb89
bb50:
  %v166 = icmp ult i32 25, 4
  %v167 = xor i1 %v166, 1
  br i1 %v167, label %bb52, label %bb51
bb51:
  %v168 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 25
  %v169 = load i32, ptr %v168, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 25, i32 %v169) #0
  br label %bb90
bb52:
  %v171 = icmp ult i32 26, 4
  %v172 = xor i1 %v171, 1
  br i1 %v172, label %bb54, label %bb53
bb53:
  %v173 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 26
  %v174 = load i32, ptr %v173, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 26, i32 %v174) #0
  br label %bb91
bb54:
  %v176 = icmp ult i32 27, 4
  %v177 = xor i1 %v176, 1
  br i1 %v177, label %bb56, label %bb55
bb55:
  %v178 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 27
  %v179 = load i32, ptr %v178, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 27, i32 %v179) #0
  br label %bb92
bb56:
  %v181 = icmp ult i32 28, 4
  %v182 = xor i1 %v181, 1
  br i1 %v182, label %bb58, label %bb57
bb57:
  %v183 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 28
  %v184 = load i32, ptr %v183, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 28, i32 %v184) #0
  br label %bb93
bb58:
  %v186 = icmp ult i32 29, 4
  %v187 = xor i1 %v186, 1
  br i1 %v187, label %bb60, label %bb59
bb59:
  %v188 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 29
  %v189 = load i32, ptr %v188, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 29, i32 %v189) #0
  br label %bb94
bb60:
  %v191 = icmp ult i32 30, 4
  %v192 = xor i1 %v191, 1
  br i1 %v192, label %bb62, label %bb61
bb61:
  %v193 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 30
  %v194 = load i32, ptr %v193, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 30, i32 %v194) #0
  br label %bb95
bb62:
  %v196 = icmp ult i32 31, 4
  %v197 = xor i1 %v196, 1
  br i1 %v197, label %bb64, label %bb63
bb63:
  %v198 = getelementptr inbounds i32, ptr %__cuda_oxide_local_x32093132380977095b7533323b2033325d_, i64 31
  %v199 = load i32, ptr %v198, align 4
  call void asm sideeffect "call _optix_set_payload, ($0, $1);", "r,r"(i32 31, i32 %v199) #0
  br label %bb96
bb64:
  ret void
bb65:
  br label %bb2
bb66:
  br label %bb4
bb67:
  br label %bb6
bb68:
  br label %bb8
bb69:
  br label %bb10
bb70:
  br label %bb12
bb71:
  br label %bb14
bb72:
  br label %bb16
bb73:
  br label %bb18
bb74:
  br label %bb20
bb75:
  br label %bb22
bb76:
  br label %bb24
bb77:
  br label %bb26
bb78:
  br label %bb28
bb79:
  br label %bb30
bb80:
  br label %bb32
bb81:
  br label %bb34
bb82:
  br label %bb36
bb83:
  br label %bb38
bb84:
  br label %bb40
bb85:
  br label %bb42
bb86:
  br label %bb44
bb87:
  br label %bb46
bb88:
  br label %bb48
bb89:
  br label %bb50
bb90:
  br label %bb52
bb91:
  br label %bb54
bb92:
  br label %bb56
bb93:
  br label %bb58
bb94:
  br label %bb60
bb95:
  br label %bb62
bb96:
  br label %bb64
}

define [32 x i32] @optix_device__raw__trace_typed_32(i32 %v0, i64 %v1, float %v2, float %v3, float %v4, float %v5, float %v6, float %v7, float %v8, float %v9, float %v10, i32 %v11, i32 %v12, i32 %v13, i32 %v14, i32 %v15, i32 %v16, i32 %v17, i32 %v18, i32 %v19, i32 %v20, i32 %v21, i32 %v22, i32 %v23, i32 %v24, i32 %v25, i32 %v26, i32 %v27, i32 %v28, i32 %v29, i32 %v30, i32 %v31, i32 %v32, i32 %v33, i32 %v34, i32 %v35, i32 %v36, i32 %v37, i32 %v38, i32 %v39, i32 %v40, i32 %v41, i32 %v42, i32 %v43, i32 %v44, i32 %v45, i32 %v46, i32 %v47, i32 %v48) alwaysinline #0 {
entry:
  %v49 = alloca [32 x i32], align 4
  br label %bb0
bb0:
  %v50 = phi i32 [ %v0, %entry ]
  %v51 = phi i64 [ %v1, %entry ]
  %v52 = phi float [ %v2, %entry ]
  %v53 = phi float [ %v3, %entry ]
  %v54 = phi float [ %v4, %entry ]
  %v55 = phi float [ %v5, %entry ]
  %v56 = phi float [ %v6, %entry ]
  %v57 = phi float [ %v7, %entry ]
  %v58 = phi float [ %v8, %entry ]
  %v59 = phi float [ %v9, %entry ]
  %v60 = phi float [ %v10, %entry ]
  %v61 = phi i32 [ %v11, %entry ]
  %v62 = phi i32 [ %v12, %entry ]
  %v63 = phi i32 [ %v13, %entry ]
  %v64 = phi i32 [ %v14, %entry ]
  %v65 = phi i32 [ %v15, %entry ]
  %v66 = phi i32 [ %v16, %entry ]
  %v67 = phi i32 [ %v17, %entry ]
  %v68 = phi i32 [ %v18, %entry ]
  %v69 = phi i32 [ %v19, %entry ]
  %v70 = phi i32 [ %v20, %entry ]
  %v71 = phi i32 [ %v21, %entry ]
  %v72 = phi i32 [ %v22, %entry ]
  %v73 = phi i32 [ %v23, %entry ]
  %v74 = phi i32 [ %v24, %entry ]
  %v75 = phi i32 [ %v25, %entry ]
  %v76 = phi i32 [ %v26, %entry ]
  %v77 = phi i32 [ %v27, %entry ]
  %v78 = phi i32 [ %v28, %entry ]
  %v79 = phi i32 [ %v29, %entry ]
  %v80 = phi i32 [ %v30, %entry ]
  %v81 = phi i32 [ %v31, %entry ]
  %v82 = phi i32 [ %v32, %entry ]
  %v83 = phi i32 [ %v33, %entry ]
  %v84 = phi i32 [ %v34, %entry ]
  %v85 = phi i32 [ %v35, %entry ]
  %v86 = phi i32 [ %v36, %entry ]
  %v87 = phi i32 [ %v37, %entry ]
  %v88 = phi i32 [ %v38, %entry ]
  %v89 = phi i32 [ %v39, %entry ]
  %v90 = phi i32 [ %v40, %entry ]
  %v91 = phi i32 [ %v41, %entry ]
  %v92 = phi i32 [ %v42, %entry ]
  %v93 = phi i32 [ %v43, %entry ]
  %v94 = phi i32 [ %v44, %entry ]
  %v95 = phi i32 [ %v45, %entry ]
  %v96 = phi i32 [ %v46, %entry ]
  %v97 = phi i32 [ %v47, %entry ]
  %v98 = phi i32 [ %v48, %entry ]
  %v99 = call { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } asm sideeffect "call($0,$1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20,$21,$22,$23,$24,$25,$26,$27,$28,$29,$30,$31),_optix_trace_typed_32,($32,$33,$34,$35,$36,$37,$38,$39,$40,$41,$42,$43,$44,$45,$46,$47,$48,$49,$50,$51,$52,$53,$54,$55,$56,$57,$58,$59,$60,$61,$62,$63,$64,$65,$66,$67,$68,$69,$70,$71,$72,$73,$74,$75,$76,$77,$78,$79,$80);", "=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,=r,r,l,f,f,f,f,f,f,f,f,f,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r,r"(i32 %v50, i64 %v51, float %v52, float %v53, float %v54, float %v55, float %v56, float %v57, float %v58, float %v59, float %v60, i32 %v61, i32 %v62, i32 %v63, i32 %v64, i32 %v65, i32 %v66, i32 %v67, i32 %v68, i32 %v69, i32 %v70, i32 %v71, i32 %v72, i32 %v73, i32 %v74, i32 %v75, i32 %v76, i32 %v77, i32 %v78, i32 %v79, i32 %v80, i32 %v81, i32 %v82, i32 %v83, i32 %v84, i32 %v85, i32 %v86, i32 %v87, i32 %v88, i32 %v89, i32 %v90, i32 %v91, i32 %v92, i32 %v93, i32 %v94, i32 %v95, i32 %v96, i32 %v97, i32 %v98) #0
  %v100 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 0
  %v101 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 1
  %v102 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 2
  %v103 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 3
  %v104 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 4
  %v105 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 5
  %v106 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 6
  %v107 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 7
  %v108 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 8
  %v109 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 9
  %v110 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 10
  %v111 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 11
  %v112 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 12
  %v113 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 13
  %v114 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 14
  %v115 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 15
  %v116 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 16
  %v117 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 17
  %v118 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 18
  %v119 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 19
  %v120 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 20
  %v121 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 21
  %v122 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 22
  %v123 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 23
  %v124 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 24
  %v125 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 25
  %v126 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 26
  %v127 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 27
  %v128 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 28
  %v129 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 29
  %v130 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 30
  %v131 = extractvalue { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } %v99, 31
  br label %bb1
bb1:
  %v132 = insertvalue [32 x i32] undef, i32 %v100, 0
  %v133 = insertvalue [32 x i32] %v132, i32 %v101, 1
  %v134 = insertvalue [32 x i32] %v133, i32 %v102, 2
  %v135 = insertvalue [32 x i32] %v134, i32 %v103, 3
  %v136 = insertvalue [32 x i32] %v135, i32 %v104, 4
  %v137 = insertvalue [32 x i32] %v136, i32 %v105, 5
  %v138 = insertvalue [32 x i32] %v137, i32 %v106, 6
  %v139 = insertvalue [32 x i32] %v138, i32 %v107, 7
  %v140 = insertvalue [32 x i32] %v139, i32 %v108, 8
  %v141 = insertvalue [32 x i32] %v140, i32 %v109, 9
  %v142 = insertvalue [32 x i32] %v141, i32 %v110, 10
  %v143 = insertvalue [32 x i32] %v142, i32 %v111, 11
  %v144 = insertvalue [32 x i32] %v143, i32 %v112, 12
  %v145 = insertvalue [32 x i32] %v144, i32 %v113, 13
  %v146 = insertvalue [32 x i32] %v145, i32 %v114, 14
  %v147 = insertvalue [32 x i32] %v146, i32 %v115, 15
  %v148 = insertvalue [32 x i32] %v147, i32 %v116, 16
  %v149 = insertvalue [32 x i32] %v148, i32 %v117, 17
  %v150 = insertvalue [32 x i32] %v149, i32 %v118, 18
  %v151 = insertvalue [32 x i32] %v150, i32 %v119, 19
  %v152 = insertvalue [32 x i32] %v151, i32 %v120, 20
  %v153 = insertvalue [32 x i32] %v152, i32 %v121, 21
  %v154 = insertvalue [32 x i32] %v153, i32 %v122, 22
  %v155 = insertvalue [32 x i32] %v154, i32 %v123, 23
  %v156 = insertvalue [32 x i32] %v155, i32 %v124, 24
  %v157 = insertvalue [32 x i32] %v156, i32 %v125, 25
  %v158 = insertvalue [32 x i32] %v157, i32 %v126, 26
  %v159 = insertvalue [32 x i32] %v158, i32 %v127, 27
  %v160 = insertvalue [32 x i32] %v159, i32 %v128, 28
  %v161 = insertvalue [32 x i32] %v160, i32 %v129, 29
  %v162 = insertvalue [32 x i32] %v161, i32 %v130, 30
  %v163 = insertvalue [32 x i32] %v162, i32 %v131, 31
  ret [32 x i32] %v163
}


@llvm.used = appending global [5 x ptr] [ptr @__closesthit__shade, ptr @__direct_callable__double, ptr @__miss__background, ptr @__raygen__render, ptr addrspacecast (ptr addrspace(4) @params to ptr)], section "llvm.metadata"

attributes #0 = { convergent }
