/* Header-only NVTX v3 shim compiled optionally by build.rs when CUDA NVTX3 headers are present.
 * Emits zero-dependency Nsight Systems range markers via NVTX_INJECTION64_PATH, collapsing to a
 * near-zero-cost null-pointer check when no profiler is attached.
 */

#include <stdint.h>
#include <string.h>

#if defined(_WIN32)
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#include <windows.h>
#elif defined(__linux__)
#include <sys/syscall.h>
#include <unistd.h>
#else
#include <pthread.h>
#endif

#include <nvtx3/nvToolsExt.h>

void nv_demo_nvtx_push(const char* name, uint32_t argb) {
    nvtxEventAttributes_t attrib;
    memset(&attrib, 0, sizeof(attrib));
    attrib.version = NVTX_VERSION;
    attrib.size = NVTX_EVENT_ATTRIB_STRUCT_SIZE;
    attrib.colorType = NVTX_COLOR_ARGB;
    attrib.color = argb;
    attrib.messageType = NVTX_MESSAGE_TYPE_ASCII;
    attrib.message.ascii = name;
    nvtxRangePushEx(&attrib);
}

void nv_demo_nvtx_pop(void) {
    nvtxRangePop();
}

void nv_demo_nvtx_name_thread(const char* name) {
#if defined(_WIN32)
    uint32_t tid = (uint32_t)GetCurrentThreadId();
#elif defined(__linux__)
    uint32_t tid = (uint32_t)syscall(SYS_gettid);
#else
    uint32_t tid = (uint32_t)(uintptr_t)pthread_self();
#endif
    nvtxNameOsThreadA(tid, name);
}
