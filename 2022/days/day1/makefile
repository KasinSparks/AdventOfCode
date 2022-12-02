cc = clang
cflags = -Wall

linker_flags = -lstdc++

modules = ../../modules/*.cpp

source_files = ./rps_player.cpp

main:
	$(cc) $(cflags) $(linker_flags) $(modules) $(source_files) main.cpp -o ./bin/main

all:
	main
