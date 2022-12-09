cc = clang
cflags = -Wall

linker_flags = -lstdc++

modules = ../../modules/*.cpp

part1: $(modules) ./part1/*cpp
	$(cc) $(cflags) $(linker_flags) $(modules) ./part1/*.cpp -o ./bin/part1

part2: $(modules) ./part2/*cpp
	$(cc) $(cflags) $(linker_flags) $(modules) ./part2/*.cpp -o ./bin/part2


all: part1 part2

clean:
	rm -f ./bin/*
