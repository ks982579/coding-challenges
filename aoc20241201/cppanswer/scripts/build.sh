#/bin/bash
# g++ -std=c++20 ./src/main.cpp -o ./target/aoc2024
# g++ src/main.cpp src/libs/asset_reader.cpp -I src -o target/MyProject -std=c++17
g++ -c src/libs/asset_reader.cpp -o asset_reader.o -std=c++20
# Add src dir to include path with -I so it can find asset_reader.h
g++ -c src/main.cpp -I src -o main.o -std=c++17
g++ main.o asset_reader.o -o target/aoc2024
