#include <iostream>
#include <sys/types.h>

#include "../../../modules/file_reader.hpp"

#include "cargo_crane.hpp"


int main() {
	std::cout << "Advent of Code Day XX!" << std::endl << std::endl;

	std::list<std::string> file_data;
	//read_file(file_data, "./data/sample_input.txt");
	read_file(file_data, "./data/input.txt");
	
	/*
	for (std::string str_data : file_data) {
		std::cout << str_data << std::endl;
	}
	*/

	uint32_t f_data_size = file_data.size();
	std::string f_data[f_data_size];
	uint32_t count = 0;
	uint32_t newline_number = 0;
	for (std::string s : file_data) {
		//std::cout << s[0] + 100 << std::endl;
		if (s[0] == '\n' || s[0] == '\0') {
			newline_number = count;
		}
		f_data[count] = s;
		count++;
	}

	struct cargo_movement_data cmd {
		f_data,
		newline_number,
		f_data_size
	};

	cargo_crane cc(cmd);
	cc.print();
}
