@echo off

pushd %~dp0

set LIBCLANG_PATH = "C:\Program Files\LLVM\bin"

mkdir gen 2> nul
bindgen ./mylibrary/include/mylibrary.hpp -o gen/bindings.rs

popd