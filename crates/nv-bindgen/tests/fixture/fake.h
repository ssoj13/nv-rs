/* Synthetic library header for nv-bindgen's own tests (no NVIDIA SDK needed). */
typedef enum fakeStatus_enum {
    FAKE_SUCCESS = 0,
    FAKE_ERROR_NOT_LOADED = 1,
    FAKE_ERROR_MISSING_SYMBOL = 2
} fakeStatus;

typedef struct fakeCtx_st* fakeCtx;
typedef struct { int err; int cuda; } fakeIoStatus;
typedef long fakeCount;

#define FAKE_VERSION 4200
#define FAKE_IO_NOT_LOADED 13
#define FAKE_IO_MISSING 17

fakeStatus fakeInit(unsigned int flags);
fakeStatus fakeCtxCreate(fakeCtx* out, int device);
fakeStatus fakeGetCount(int* count);
int notFakeHelper(void);
fakeStatus fakeShadow(int f, int api);
const char* fakeErrorString(fakeStatus status);
void fakeReset(void);
void fakeStore(int* value);
int fakeVersionNumber(void);
int fakeMissingNumber(void);
fakeIoStatus fakeIo(void);
fakeCount fakeTransfer(void);
