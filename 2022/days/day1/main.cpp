#include <iostream>
#include <sys/types.h>

#include "../../modules/file_reader.hpp"
#include "elf.hpp"


int main() {
	std::cout << "Advent of Code Day 0!" << std::endl << std::endl;

	//std::list<std::string> test = read_file("./data/sample_input.txt");
	std::list<std::string> test = read_file("./data/input.txt");
	std::list<elf*> elfs;
	
	elf* current_elf = new elf();

	for(std::string t : test) {
		if (t.empty()) {
			// Next elf
			elfs.push_back(current_elf);
			current_elf = new elf();
		} else {
			current_elf->add_food(std::stoi(t));
		}
	}

	// Include the last elf
	elfs.push_back(current_elf);

	// Find the top three elfs
	//elfs.sort();
	elfs.sort([](elf* lhs, elf* rhs) {return lhs->get_total_calories() < rhs->get_total_calories();});
	
	for (elf* e : elfs) {
		std::cout << e->get_total_calories() << std::endl;
	}
	
	std::list<elf*>::iterator it = elfs.end();
	it--;
	u_int8_t count = 0;

	u_int32_t calories_max = 0;
	while (it != elfs.begin()) {
		if (count >= 3) {
			break;
		}
		std::cout << (*it)->get_total_calories() << std::endl;
		calories_max += (*it)->get_total_calories();
		it--;
		count++;
	}

	std::cout << "Top Three Max Calories: " << calories_max << std::endl;


	for (elf* e : elfs) {
		e->~elf();
		delete e;
	}
}
