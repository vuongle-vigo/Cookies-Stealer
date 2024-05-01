#pragma once

# include "utils.hpp"

class ChromeDecoder {
protected:
	BCRYPT_KEY_HANDLE m_hKey;
	BYTE bDecryptedKey[8192];
	DWORD dwKeysize;

public:
	ChromeDecoder();
	~ChromeDecoder();
	bool GenerateKey(LPVOID lpJsonBuffer);
	BCRYPT_KEY_HANDLE GetKeyHandle();
};