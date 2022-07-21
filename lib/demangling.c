#pragma comment(lib, "DbgHelp.lib")

#include "Windows.h"
#include "DbgHelp.h"

const char* demangle(const char* symbol)
{
    char buffer[1024];
    memset(buffer, 0, 1024);
    UnDecorateSymbolName(symbol, buffer, 1024, 0);
    return buffer;
}