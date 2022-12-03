#include <iostream>
#include <sys/types.h>

#include "../../modules/file_reader.hpp"
#include "rucksack.hpp"


int main() {
	std::cout << "Advent of Code Day 03!" << std::endl << std::endl;

	std::list<std::string> file_data;
	//read_file(file_data, "./data/sample_input.txt");
	read_file(file_data, "./data/input.txt");
	
	uint32_t total = 0;

	
	uint32_t count = 0;
	rucksack rs;
	for (std::string str_data : file_data) {
		//std::cout << str_data << std::endl;
		if (count >= 3) {
			std::cout << "Item priority: " << rs.get_item_priority() << std::endl;
			total += rs.get_item_priority();
			rs = rucksack();
			count = 0;
		}

		rs.add_line(str_data);
		count++;
	}

	// get the last one
	std::cout << "Item priority: " << rs.get_item_priority() << std::endl;
	total += rs.get_item_priority();

	std::cout << "Total: " << total << std::endl;
}
