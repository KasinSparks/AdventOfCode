#cc = clang++
cc = g++
cflags = -Wall

linker_flags = -lstdc++

modules = ../../modules/*.cpp

source_files = 

part1: $(modules) ./part1/*.cpp
	$(cc) $(cflags) $(linker_flags) ./part1/*.hpp  $(modules) ./part2/tree_house.cpp ./part1/main.cpp -o ./bin/part1

part2: $(modules) ./part2/*cpp
	$(cc) $(cflags) $(linker_flags) ./part2/*.hpp  $(modules) ./part2/tree_house.cpp ./part2/main.cpp -o ./bin/part2


all: part1 part2

clean:
	rm -f ./bin/*
