/* Synthetic library header for nv-bindgen's own tests (no NVIDIA SDK needed). */
typedef enum fakeStatus_enum {
    FAKE_SUCCESS = 0,
    FAKE_ERROR_NOT_LOADED = 1,
    FAKE_ERROR_MISSING_SYMBOL = 2
} fakeStatus;

typedef struct fakeCtx_st* fakeCtx;

#define FAKE_VERSION 4200

fakeStatus fakeInit(unsigned int flags);
fakeStatus fakeCtxCreate(fakeCtx* out, int device);
fakeStatus fakeGetCount(int* count);
int notFakeHelper(void);
const char* fakeErrorString(fakeStatus status);
void fakeReset(void);
int fakeVersionNumber(void);
