#include "rucksack.hpp"
#include <cstdint>
#include <exception>
#include <unordered_map>
#include <iostream>
#include <utility>

rucksack::rucksack(const std::string &line) {
	uint32_t half_point = line.size() / 2;
	
	// Create a hashtable to group the two pairs of rucksacks and find the 
	// duplicate item
	std::unordered_map<char, uint8_t> map;
	
	// Add the first rucksack's contents to the hashtable
	for (int i = 0; i < half_point; ++i) {
		map.insert(std::pair<char, uint8_t>(line[i], 1));
	}
	

	// Add the seconds rucksack contents
	for (int i = half_point; i < line.size(); ++i) {
		auto search = map.find(line[i]);
		if (search != map.end()) {
			//std::cout << search->first << std::endl;
			this->_item = search->first;
			break;
		}
	}
	
}

uint32_t
rucksack::get_duplicate_item_priority() {
	uint32_t value = 0;
	if (this->_item >= 'A' && this->_item <= 'Z') {
		value = (this->_item - 'A') + 27;
	} else if (this->_item >= 'a' && this->_item <= 'z') {
		value = (this->_item - 'a') + 1;
	} else {
		throw new std::exception();
	}

	return value;
}
