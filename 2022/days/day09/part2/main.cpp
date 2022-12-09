#include <cstdint>
#include <exception>
#include <iostream>

#include "../../../modules/file_reader.hpp"
#include "rope.hpp"

std::pair<aoc_direction, uint64_t> parse(const std::string &data) {
	std::pair<aoc_direction, uint64_t> p;
	// Read first char for direction
	switch (data.at(0)) {
		case 'U':
			p.first = aoc_direction::NORTH;
			break;
		case 'D':
			p.first = aoc_direction::SOUTH;
			break;
		case 'R':
			p.first = aoc_direction::EAST;
			break;
		case 'L':
			p.first = aoc_direction::WEST;
			break;
		default:
			throw new std::exception();
			break;
	}

	p.second = std::stoi(data.substr(2));

	return p;
}

int main() {
	std::cout << "Advent of Code Day 09!" << std::endl << std::endl;

	std::list<std::string> file_data;
	//read_file(file_data, "./data/sample_input.txt");
	//read_file(file_data, "./data/sample_input2.txt");
	read_file(file_data, "./data/input.txt");
	
	rope r;

	for (std::string str_data : file_data) {
		std::cout << str_data << std::endl;
		std::pair<aoc_direction, int64_t> p = parse(str_data);
		//std::cout << "Direction: " << p.first << ", amount: " << p.second << std::endl;
		r.add_movement(p.first, p.second);
	}

	std::cout << "Total: " << r.get_num_of_tail_visited() << std::endl;

	//r.print_hashtable();
}
