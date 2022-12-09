#include <cstdint>
#include <iostream>
#include <sys/types.h>

#include "../../../modules/file_reader.hpp"

#include "aoc_grid.hpp"
#include "tree_house.hpp"


int main() {
	std::cout << "Advent of Code Day 08!" << std::endl << std::endl;

	std::list<std::string> file_data;
	//read_file(file_data, "./data/sample_input.txt");
	read_file(file_data, "./data/input.txt");
	
	uint32_t line_size = file_data.begin()->size();

	tree_house th(line_size, file_data.size());
	
	for (std::string str_data : file_data) {
		std::cout << str_data << std::endl;
	}

	auto it = file_data.begin();
	int i = 0;
	for (auto it = file_data.begin(); it != file_data.end(); ++it) {
		for (int j = 0; j < line_size; ++j) {
			th.set(j, i, (it)->at(j) - '0');
		}
		i++;
	}

	std::cout << "count: " << i << std::endl;

	std::cout << "DATA:" << std::endl;
	th.print();

	aoc_grid<uint32_t> test_grid(th.get_width(), th.get_height());
	
	int num_visible_trees = 0;
	for (int i = 0; i < th.get_width(); ++i) {
		for (int j = 0; j < th.get_height(); ++j) {
			//std::cout << "CHecking:" << std::endl;
			if ( th.is_visible_from_edge(i, j, aoc_direction::NORTH, th.get(i, j))
				|| th.is_visible_from_edge(i, j, aoc_direction::SOUTH, th.get(i, j))
			 	|| th.is_visible_from_edge(i, j, aoc_direction::EAST, th.get(i, j)) 
			 	|| th.is_visible_from_edge(i, j, aoc_direction::WEST, th.get(i, j)) 
			 ) {
				num_visible_trees++;
				std::cout << "\033[1;31m" << "Tree Visible: (" << i << "," << j << ") VAL: " <<
					th.get(i, j) << "\033[1;37m" << std::endl;
				test_grid.set(i, j, th.get(i, j));
			}
		}
	}
	
	std::cout << "\033[1;31m" << std::endl;
	test_grid.print();
	std::cout << "\033[1;37m" << std::endl;;

	std::cout << "Total: " << num_visible_trees << std::endl;
}
