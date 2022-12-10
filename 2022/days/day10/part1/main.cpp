#include <cstdint>
#include <iostream>
#include <sys/types.h>

#include "../../../modules/file_reader.hpp"
#include "aoc_device.hpp"


int main() {
	std::cout << "Advent of Code Day 10!" << std::endl << std::endl;

	std::list<std::string> file_data;
	//read_file(file_data, "./data/sample_input.txt");
	//read_file(file_data, "./data/sample_input2.txt");
	read_file(file_data, "./data/input.txt");
	
	int64_t total = 0;
	aoc_device device;

	for (std::string str_data : file_data) {
		//std::cout << str_data << std::endl;
		device.parse(str_data);

		if (device.get_report_status()) {
			total += device.get_signal_strength();
			device.set_report_status_false();
		}
	}

	std::cout << "Total: " << total << std::endl;
}
