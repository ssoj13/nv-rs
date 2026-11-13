/* Id table (like nvapi_interface.h). */
struct QI_TABLE { const char* func; unsigned int id; };
struct QI_TABLE qi_table[] =
{
    { "qiInitialize", 0x0150e828 },
    { "qiGetValue", 0xABCDEF01 },
};
