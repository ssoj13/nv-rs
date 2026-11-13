/* Query-interface fixture (like nvapi.h): functions reached by id. */
typedef enum _qiStatus { QI_OK = 0, QI_LIBRARY_NOT_FOUND = -2, QI_NO_IMPLEMENTATION = -3 } qiStatus;
qiStatus qiInitialize(void);
qiStatus qiGetValue(int* f, int api);
qiStatus qiNotInTable(void);
