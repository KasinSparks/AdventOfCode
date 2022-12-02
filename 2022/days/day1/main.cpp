#include <iostream>
#include <sys/types.h>

#include "../../modules/file_reader.hpp"
#include "rps_player.hpp"


int main() {
	std::cout << "Advent of Code Day 02!" << std::endl << std::endl;

	std::list<std::string> file_data;
	//read_file(file_data, "./data/sample_input.txt");
	read_file(file_data, "./data/input.txt");
	
	uint32_t total_score = 0;

	for (std::string str_data : file_data) {
		rps_player rps;
		uint16_t score = rps.play_round(str_data);
		std::cout << score << std::endl;
		total_score += score;

		//std::cout << str_data << std::endl;
	}

	std::cout << "Total: " << total_score << std::endl;

		
}
