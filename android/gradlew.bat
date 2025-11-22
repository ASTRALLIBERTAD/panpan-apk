@echo off
where gradle >nul 2>&1
if %errorlevel% neq 0 (
  echo Gradle not found. Please install Gradle or replace this with a real gradle wrapper.
  exit /b 1
)
gradle %*
