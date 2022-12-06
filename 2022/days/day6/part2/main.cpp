#include <iostream>
#include <sys/types.h>

#include "../../../modules/file_reader.hpp"

#include "marker.hpp"


int main() {
	std::cout << "Advent of Code Day 06!" << std::endl << std::endl;

	std::list<std::string> file_data;
	//read_file(file_data, "./data/sample_input.txt");
	read_file(file_data, "./data/input.txt");
	
	for (std::string str_data : file_data) {
		std::cout << str_data << std::endl;
		marker m(str_data);
		std::cout << "Found dup at: " << m.find_mark() << std::endl;
	}
}
