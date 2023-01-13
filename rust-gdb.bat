@echo off
SET PYTHONPATH=%PYTHONHOME%\Lib

SET RUST_GDB=arm-none-eabi-gdb-py.exe

for /f %%s in ('rustc --print=sysroot') do SET RUSTC_SYSROOT=%%s

REM appearently %errorlevel% does not get set in code above, so let's check for RUSTC_SYSROOT
if not defined RUSTC_SYSROOT (  exit /b 1 )
SET GDB_PYTHON_MODULE_DIRECTORY=%RUSTC_SYSROOT%\lib\rustlib\etc

REM  Set the environment variable `RUST_GDB` to overwrite the call to a
REM  different/specific command (defaults to `gdb`).
if not defined RUST_GDB ( SET RUST_GDB=gdb )

if not defined PYTHONPATH ( 
	SET PYTHONPATH=%GDB_PYTHON_MODULE_DIRECTORY%
) else (
	SET PYTHONPATH=%PYTHONPATH%;%GDB_PYTHON_MODULE_DIRECTORY%
)

%RUST_GDB% --directory="%GDB_PYTHON_MODULE_DIRECTORY%" -iex "add-auto-load-safe-path %GDB_PYTHON_MODULE_DIRECTORY%" %*
