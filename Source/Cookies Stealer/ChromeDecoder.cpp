#include "ChromeDecoder.hpp"

ChromeDecoder::ChromeDecoder() {
	this->dwKeysize = 0;
	this->m_hKey = NULL;
}

ChromeDecoder::~ChromeDecoder() {
	CloseHandle(this->m_hKey);
}

BCRYPT_KEY_HANDLE ChromeDecoder::GetKeyHandle() {
	return this->m_hKey;
}

bool ChromeDecoder::GenerateKey(LPVOID lpJsonBuffer) 
{
	const auto jsonData = cJSON_Parse((char*)lpJsonBuffer);
	if (!jsonData) 
	{
		//printf("cJSON_Parse");
		return false;
	}

	cJSON* node = jsonData->child;
	//os_crypt
	while (strcmp(node->string, "os_crypt")) 
	{
		if (!node) 
		{
			//printf("jsonData->child");
			return false;
		}
		node = node->next;
	}
	node = node->child;
	//encrypted_key
	while (strcmp(node->string, "encrypted_key")) 
	{
		if (!node) 
		{
			printf("jsonData->child");
			return false;
		}
		node = node->next;
	}

	const auto keyBase64 = cJSON_GetStringValue(node);
	int keySize = strlen(keyBase64);

	char decryptedKey[8192] = { 0 };

	char* keyEncDPAPI = NULL;
	DWORD keyEncDPAPISize = 0;
	BYTE* keyEnc = NULL;
	DWORD keyEncSize = 0;

	keyEncDPAPI = new char[keySize];

	auto key64Decoded = base64_decryptor::base64_decode(keyBase64);
	keyEncDPAPISize = key64Decoded.length();

	keyEncDPAPI = (char*)key64Decoded.c_str();
	PVOID x = &keyEncDPAPI;
	keyEnc = new BYTE[keyEncDPAPISize - DPAPI_PREFIX_LEN];

	int counter = 0;
	for (int i = DPAPI_PREFIX_LEN; i < keyEncDPAPISize; i++)
	{
		keyEnc[counter++] = (BYTE)keyEncDPAPI[i];
	}

	DATA_BLOB in = { 0 };
	DATA_BLOB out = { 0 };

	in.pbData = keyEnc;
	in.cbData = keyEncDPAPISize;
	int sizeKey = 0;
	if (CryptUnprotectData(&in, NULL, NULL, NULL, NULL, 0, &out)) {
		for (int i = 0; i < out.cbData; i++) {
			decryptedKey[i] = out.pbData[i];
			sizeKey++;
		}

		decryptedKey[out.cbData] = '\0';
	}

	BCRYPT_ALG_HANDLE hAlg = NULL;
	do 
	{
		if (BCryptOpenAlgorithmProvider(&hAlg, BCRYPT_AES_ALGORITHM, NULL, 0) != 0)
		{
			/*printf("[DEBUG] Crypt::BCrypt::Init: can't initialize cryptoprovider. Last error code: %d \n",
				GetLastError());*/
			break;
		}
		if (BCryptSetProperty(hAlg, BCRYPT_CHAINING_MODE, (PUCHAR)BCRYPT_CHAIN_MODE_GCM,
			sizeof(BCRYPT_CHAIN_MODE_GCM),
			0) != 0)
		{
			printf("[DEBUG] Crypt::BCrypt::Init: can't set chaining mode. Last error code: %d \n", GetLastError());
			break;
		}
	} while (false);

	NTSTATUS ntStatus = BCryptGenerateSymmetricKey(hAlg, &this->m_hKey, NULL, 0, (PBYTE)decryptedKey, sizeKey, 0);
	if (ntStatus != 0) {
		//printf("[DEBUG] Crypt::BCrypt::Init: can't deinitialize cryptoprovider. Last error code: %x \n", ntStatus);
	}
}