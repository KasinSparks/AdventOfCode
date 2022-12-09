#include <cstdint>
#include <iostream>
#include <list>
#include <string>

#include "../../../modules/file_reader.hpp"

#include "assignments.hpp"


int main() {
	std::cout << "Advent of Code Day 04!" << std::endl << std::endl;

	std::list<std::string> file_data;
	//read_file(file_data, "./data/sample_input.txt");
	read_file(file_data, "./data/input.txt");
	
	std::list<assignment> elf_assignments;

	for (std::string str_data : file_data) {
		// Parse the data
		//std::cout << str_data << std::endl;
		std::string elf_assignment_data[4] = {"", "", "", ""};
		uint8_t count = 0;

		for (char c : str_data) {
			if (c == '-' || c == ',') {
				count++;
				continue;
			}
			
			elf_assignment_data[count].push_back(c);
		}
		
		for (int i = 0; i < 4; i += 2) {
			elf_assignments.push_back(assignment(std::stoi(elf_assignment_data[i]), 
					std::stoi(elf_assignment_data[i+1])));
		}
	}

	assignment assignment_arr[elf_assignments.size()];
	int count = 0;
	for (assignment a : elf_assignments) {
		assignment_arr[count] = assignment(a);
		count++;
	}

	uint32_t total = 0;

	for (int i = 0; i < elf_assignments.size(); i += 2) { 
		if (assignment_arr[i].fully_contains(assignment_arr[i+1]) ||
					assignment_arr[i+1].fully_contains(assignment_arr[i])) {
			total++;
		}
	}

	std::cout << "Total: " << total << std::endl;
}
