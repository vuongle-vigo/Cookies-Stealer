#define _CRT_SECURE_NO_WARNINGS

#include <iostream>
#include <string>

#include <Windows.h>
#include <Shlobj.h>
#include <Shlobj_core.h>
#include <dpapi.h>
#include <bcrypt.h>

#include "sqlite3.h"
#include "cJSON.h"
#include "cJSON_Utils.h"
#include "Base64.h"



#pragma comment(lib, "Crypt32.lib")
#pragma comment(lib, "bcrypt.lib")



using namespace std;

#define DPAPI_PREFIX_LEN 5