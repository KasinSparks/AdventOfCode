#include "rucksack.hpp"
#include <cstdint>
#include <exception>
#include <unordered_map>
#include <iostream>

rucksack::rucksack() {
	this->_items = std::unordered_map<char, uint8_t>();
}

uint32_t
rucksack::get_item_priority() {
	char item = 0;
	// Find the item that appears 3 times
	//std::cout << this->_items.size() << std::endl;
	for (std::pair<char, uint8_t> m : this->_items) {
		//std::cout << m.first << m.second << std::endl;
		if (m.second == 3) {
			item = m.first;
			break;
		}
	}

	if (item == 0) {
		std::cout << "ERROR:: NO ITEM APPEARS 3 TIMES!" << std::endl;
		for (std::pair<char, uint8_t> i : this->_items) {
			std::cout << "<" << i.first << ", " << int (i.second) << ">" << std::endl;
		}
		throw new std::exception();
	}

	uint32_t value = 0;
	if (item >= 'A' && item <= 'Z') {
		value = (item - 'A') + 27;
	} else if (item >= 'a' && item <= 'z') {
		value = (item - 'a') + 1;
	} else {
		throw new std::exception();
	}

	return value;
}

void
rucksack::add_line(const std::string &line) {
	std::unordered_map<char, uint8_t> map;

	map = this->_remove_duplicates(line);
	//std::cout << "map size: " << map.size() << std::endl;
	
	for (std::pair<char, uint8_t> p : map) {
		auto search = this->_items.find(p.first);
		if (search != this->_items.end()) {
			// Found the item. Increase the count
			search->second += 1;
			//this->_items = search->first;
		} else {
			// Item does not exist in the hashmap, add it now!
			this->_items.insert(std::pair<char, uint8_t>(p.first, 1));
		}
	}
}
