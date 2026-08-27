@echo off
:: Wrapper for cdd (Windows Command Prompt - CMD)
:: Usage: 
:: If this script is executed, it will intercept the output of the Rust binary 
:: and change the directory of the current CMD session.

set "CDD_DIR=%~dp0"
set "CDD_BIN="

if exist "%CDD_DIR%cdd.exe" (
    set "CDD_BIN=%CDD_DIR%cdd.exe"
) else if exist "%CDD_DIR%..\..\core\target\release\cdd.exe" (
    set "CDD_BIN=%CDD_DIR%..\..\core\target\release\cdd.exe"
) else if exist "%CDD_DIR%..\..\core\target\debug\cdd.exe" (
    set "CDD_BIN=%CDD_DIR%..\..\core\target\debug\cdd.exe"
) else (
    echo cdd: command not found. Ensure the Rust binary is compiled.
    exit /b 1
)

set "TMP_FILE=%TEMP%\cdd_out_%RANDOM%.txt"

"%CDD_BIN%" %* --cdd-out-file "%TMP_FILE%"
set CDD_EXIT=%ERRORLEVEL%

if %CDD_EXIT% EQU 0 (
    if exist "%TMP_FILE%" (
        for /f "usebackq delims=" %%A in ("%TMP_FILE%") do (
            cd /d "%%A"
        )
    )
)

if exist "%TMP_FILE%" del "%TMP_FILE%"
exit /b %CDD_EXIT%
