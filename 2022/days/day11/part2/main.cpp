#include <cstdint>
#include <iostream>
#include <list>
#include <numeric>

#include "../../../modules/file_reader.hpp"

#include "aoc_monkey.hpp"


int main() {
	std::cout << "Advent of Code Day 11!" << std::endl << std::endl;

	std::list<std::string> file_data;
	//read_file(file_data, "./data/sample_input.txt");
	read_file(file_data, "./data/input.txt");
	
	// Have to take into account there is no blank line at the bottom
	int num_of_monkeys = ((file_data.size() + 1) / 7);
	aoc_monkey monkeys[num_of_monkeys];

	std::string lines[file_data.size()];
	
	int count = 0;
	for (std::string str_data : file_data) {
		std::cout << str_data << std::endl;
		lines[count++] = str_data;
	}

	for (int i = 0; i < count; i += 7) {
		monkeys[i/7].parse_input_block(&(lines[i]));
		monkeys[i/7].print();
	}

	uint64_t lcm = 1;
	for (int i = 0; i < num_of_monkeys; ++i) {
		lcm *= monkeys[i].get_test_val();
	}
	
	for (int rounds = 0; rounds < 10000; ++rounds) {
		for (int i = 0; i < num_of_monkeys; ++i) {
			std::list<aoc_monkey_thrown_item> thrown_items;
			monkeys[i].run(&thrown_items, lcm);
			for (aoc_monkey_thrown_item item : thrown_items) {
				//std::cout << "Monkey id: " << item.dest_monkey_id
				//	<< ", Payload: " << item.payload << std::endl;
				monkeys[item.dest_monkey_id].add(item.payload);
			}
		}
		if ((rounds + 1) % 1000 == 0 || (rounds + 1) == 20 || rounds == 0) {
			std::cout << "Round: " << rounds + 1 << std::endl;
			std::list<uint64_t> inspection_nums;
			for (int i = 0; i < num_of_monkeys; ++i) {
				std::cout << "Monkey " << i << " inspected items " << 
					monkeys[i].get_num_inspected() << " times." << std::endl;
				inspection_nums.push_back(monkeys[i].get_num_inspected());
		}
	}
	}

	std::cout << std::endl << std::endl;

	for (int i = 0; i < num_of_monkeys; ++i) {
		monkeys[i].print();
	}
	
	std::list<uint64_t> inspection_nums;
	for (int i = 0; i < num_of_monkeys; ++i) {
		std::cout << "Monkey " << i << " inspected items " << 
			monkeys[i].get_num_inspected() << " times." << std::endl;
		inspection_nums.push_back(monkeys[i].get_num_inspected());
	}
	
	inspection_nums.sort();
	
	auto h = inspection_nums.end();
	//std::cout << "Highest two: " << *(--h) << ", " << *(--h) << std::endl;
	std::cout << "Level of monkey business: " << *(--h) * *(--h) << std::endl;
}
