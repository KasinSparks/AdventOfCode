#include <cstdint>
#include <exception>
#include <iostream>
#include <string>
#include <sys/types.h>

#include "../../../modules/file_reader.hpp"

#include "directory.hpp"

int main() {
	std::cout << "Advent of Code Day 07!" << std::endl << std::endl;

	std::list<std::string> file_data;
	//read_file(file_data, "./data/my_sample_input.txt");
	//read_file(file_data, "./data/sample_input.txt");
	read_file(file_data, "./data/input.txt");
	
	int ingore_first_line = 1;

	aoc_directory root_dir("/", nullptr);

	aoc_directory *current_dir;
	current_dir = &root_dir;
	// Parse the data
	for (std::string str_data : file_data) {
		// We do not need the first line
		if (ingore_first_line) {
			ingore_first_line = 0;
			continue;
		}
		std::cout << str_data << std::endl;
		std::cout << "curr dir: " << current_dir->get_name() << std::endl;
		std::string command = str_data.substr(0, 3);
		if (command == "$ c")  {
			// cd command
			if (current_dir == nullptr) {
				std::cerr << "NO CURRENT DIRECTORY!" << std::endl;
				throw new std::exception();
			}

			// if going up a directory
			if (str_data.substr(5) == "..") {
				current_dir = current_dir->get_parent();
			} else {
				current_dir->add_dir(aoc_directory(str_data.substr(5), current_dir));
				current_dir = current_dir->get_directory(str_data.substr(5));
			}
		} else if (command == "$ l") {
			// ls command
			// Read until another ls or cd
			continue;
		} else if (command == "dir") {
			// dir command
			std::string n = str_data.substr(4);
			// Add the DIRECTORY
			current_dir->add_dir(aoc_directory(n, current_dir));
		} else {
			// file
			// Read until space
			int pos = str_data.size();
			for (int i = 0; i < str_data.size(); ++i) {
				if (str_data[i] == ' ') {
					pos = i;
					break;
				}
			}

			if (pos == str_data.size()) {
				std::cerr << "DID NOT FIND A FILE NAME!" << std::endl;
				throw new std::exception();
			}

			uint32_t f_size = stoi(str_data.substr(0, pos));
			std::string f_name = str_data.substr(pos + 1);

			current_dir->add_file(aoc_file(f_size, f_name));
		}
	}

	uint64_t total_space =  70000000;
	uint64_t needed_unused_space = 30000000;
	uint64_t curr_space = total_space - root_dir.calculate_total_size();
	uint64_t needed_space = needed_unused_space - curr_space;

	std::cout << "Current: " << curr_space << " space!" << std::endl;
	std::cout << "Need: " << needed_space << " of space!" << std::endl;

	const aoc_directory *target_delete_dir = root_dir.get_directory_to_delete(needed_space);

	std::cout << "Need to delete: " << target_delete_dir->get_name() << std::endl;

	std::cout << "Total: " << target_delete_dir->calculate_total_size() << std::endl;
	//std::cout << root_dir.calculate_total_size() << std::endl;
	//std::cout << root_dir.calculate_total_with_dups() << std::endl;
}
