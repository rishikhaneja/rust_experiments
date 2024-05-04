cmake -S . -B build_vs2019 -G "Visual Studio 16 2019"
cmake --build build_vs2019 --config Debug
cmake --install build_vs2019 --config Debug