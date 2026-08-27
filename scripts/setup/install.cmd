@echo off
cd /d "%~dp0"
cls
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0install.ps1" %*
