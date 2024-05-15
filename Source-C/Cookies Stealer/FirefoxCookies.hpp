#pragma once

#include "utils.hpp"

class FirefoxCookies {
protected:
	char m_sFilePath[MAX_PATH];
	sqlite3_stmt* m_stmt;

public:
	FirefoxCookies(char* sFilePath);
	~FirefoxCookies();
	bool QuerySqlite();
};

