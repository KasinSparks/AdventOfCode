#include <iostream>
#include <sys/types.h>

#include "../../modules/file_reader.hpp"


int main() {
	std::cout << "Advent of Code Day XX!" << std::endl << std::endl;

	std::list<std::string> file_data;
	read_file(file_data, "./data/sample_input.txt");
	//read_file(file_data, "./data/input.txt");

	for (std::string str_data : file_data) {
		std::cout << str_data << std::endl;
	}
}
