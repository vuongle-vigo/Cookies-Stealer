#pragma once

#include "utils.hpp"

inline bool GetFilePath(_In_ const char* sBrowserName, _In_ const char* sFileName, _In_ int csidl, _Out_ char* sFilePath) {
	if (SHGetFolderPathA(NULL, csidl, NULL, 0, sFilePath) != S_OK) {
		return false;
	}
	strcat(sFilePath, sBrowserName);
	strcat(sFilePath, sFileName);
	return true;
}

inline bool IsFileExists(_In_ const char* sFilePath){
    DWORD fileAttributes = GetFileAttributesA(sFilePath);
	if (fileAttributes == INVALID_FILE_ATTRIBUTES) {
		DWORD error = GetLastError();
		if (error == ERROR_FILE_NOT_FOUND || error == ERROR_PATH_NOT_FOUND) {
			return false;
		}
	}
	return true;
}

inline bool ReadFile2Buffer(_In_ char* sFilePath,_Out_ LPVOID* lpBuffer, _Out_ DWORD* dwFileSize) {
	if (!IsFileExists(sFilePath)) {
		return false;
	}
	HANDLE hFile = CreateFileA(sFilePath, GENERIC_READ, 0, 0, OPEN_ALWAYS, 0, 0);
	if (hFile == INVALID_HANDLE_VALUE) {
		return false;
	}
	*dwFileSize = GetFileSize(hFile, NULL);
	if (dwFileSize == 0) {
		return false;
	}

	*lpBuffer = HeapAlloc(GetProcessHeap(), 0, *dwFileSize);

	DWORD bytesRead;
	if (!ReadFile(hFile, *lpBuffer, *dwFileSize, &bytesRead, NULL)) {
		return false;
	}
	CloseHandle(hFile);
}

void SearchFile(_In_ const char* directory, _In_ const char* fileName, _Out_ char* sFilePath) {
	char searchPath[MAX_PATH];
	sprintf(searchPath, "%s\\*", directory);

	WIN32_FIND_DATAA findData;
	HANDLE hFind = FindFirstFileA(searchPath, &findData);
	if (hFind != INVALID_HANDLE_VALUE) {
		do {
			if (strcmp(findData.cFileName, ".") != 0 && strcmp(findData.cFileName, "..") != 0) {
				char filePath[MAX_PATH];
				sprintf(filePath, "%s\\%s", directory, findData.cFileName);

				if (findData.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) {
					SearchFile(filePath, fileName, sFilePath);
				}
				else {
					if (strcmp(findData.cFileName, fileName) == 0) {
						FILE* file = fopen(filePath, "rb");
						if (file != NULL) {
							fseek(file, 0, SEEK_END);
							long size = ftell(file);
							fclose(file);

							if (size == 0) {
								break;
							}
							else {
								strcpy(sFilePath, filePath);
							}
						}
						
					}
				}
			}
		} while (FindNextFileA(hFind, &findData) != 0);
		FindClose(hFind);
	}
}