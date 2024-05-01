#include "utils.hpp"
#include "FileOperator.hpp"
#include "ChromeDecoder.hpp"
#include "ChromeCookies.hpp"
#include "FirefoxCookies.hpp"




int main() {
	char szChromeSqlitePath[MAX_PATH] = { 0 };
	// "\\Google\\Chrome", "\\User Data\\Default\\Network\\Cookies"
	if (!GetFilePath("\\Google\\Chrome", "\\User Data\\Default\\Network\\Cookies", CSIDL_LOCAL_APPDATA, szChromeSqlitePath)) {
		printf("GetFilePath for GoogleChrome Cookies Failed\n");
		return -1;
	}

	char szChromeKeyPath[MAX_PATH] = { 0 };
	// "\\Google\\Chrome", "\\User Data\\Local State"
	if (!GetFilePath("\\Google\\Chrome", "\\User Data\\Local State", CSIDL_LOCAL_APPDATA, szChromeKeyPath)) {
		printf("GetFilePath for GoogleChrome Cookies Failed\n");
		return -1;
	}

	DWORD dwFilesize = 0;
	LPVOID lpBuffer = nullptr;
	if (!ReadFile2Buffer(szChromeKeyPath, &lpBuffer, &dwFilesize)) {
		printf("ReadFile2Buffer Failed\n");
		return -1;
	}

	ChromeDecoder* chromeDecoder = new ChromeDecoder();
	if (!chromeDecoder->GenerateKey(lpBuffer))
	{
		printf("GenerateKey Failed\n");
		return -1;
	}

	ChromeCookies* chromeCookies = new ChromeCookies(szChromeSqlitePath);
	chromeCookies->QuerySqlite();
	chromeCookies->CookiesDecrypt(chromeDecoder->GetKeyHandle());

	HeapFree(GetProcessHeap(), 0, lpBuffer);

	/*char szFirefoxPath[MAX_PATH] = { 0 };
	if (!GetFilePath("\\Mozilla\\Firefox", "\\Profiles", CSIDL_APPDATA, szFirefoxPath)) {
		printf("GetFilePath for GoogleChrome Cookies Failed\n");
		return -1;
	}
	char szFirefoxSqlitePath[MAX_PATH] = { 0 };
	SearchFile(szFirefoxPath, "cookies.sqlite", szFirefoxSqlitePath);
	FirefoxCookies* firefoxCookies = new FirefoxCookies(szFirefoxSqlitePath);
	firefoxCookies->QuerySqlite();*/


	//char szCocCocKeyPath[MAX_PATH] = { 0 };
	//// "\\Google\\Chrome", "\\User Data\\Local State"
	//if (!GetFilePath("\\CocCoc\\Browser", "\\User Data\\Local State", CSIDL_LOCAL_APPDATA, szCocCocKeyPath)) {
	//	printf("GetFilePath for GoogleChrome Cookies Failed\n");
	//	return -1;
	//}

	//DWORD dwFilesize = 0;
	//LPVOID lpBuffer = nullptr;
	//if (!ReadFile2Buffer(szCocCocKeyPath, &lpBuffer, &dwFilesize)) {
	//	printf("ReadFile2Buffer Failed\n");
	//	return -1;
	//}

	//ChromeDecoder* chromeDecoder = new ChromeDecoder();
	//chromeDecoder->GenerateKey(lpBuffer);
	//chromeDecoder->GetKeyHandle();

	//char szCocCocSqlitePath[MAX_PATH] = { 0 };
	//// "\\Google\\Chrome", "\\User Data\\Default\\Network\\Cookies"
	//if (!GetFilePath("\\CocCoc\\Browser", "\\User Data\\Default\\Network\\Cookies", CSIDL_LOCAL_APPDATA, szCocCocSqlitePath)) {
	//	printf("GetFilePath for GoogleChrome Cookies Failed\n");
	//	return -1;
	//}

	//ChromeCookies* chromeCookies = new ChromeCookies(szCocCocSqlitePath);
	//chromeCookies->QuerySqlite();
	//chromeCookies->CookiesDecrypt(chromeDecoder->GetKeyHandle());


}