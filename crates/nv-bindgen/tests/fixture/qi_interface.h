/* Id table (like nvapi_interface.h). */
struct QI_TABLE { const char* func; unsigned int id; };
struct QI_TABLE qi_table[] =
{
    { "qiInitialize", 0x0150e828 },
    { "qiGetValue", 0xABCDEF01 },
    { "qiVersionNumber", 0x12345678 },
    { "qiAbsentNumber", 0x12345679 },
    { "qiStore", 0x12345680 },
    { "qiMissingStore", 0x12345681 },
};
