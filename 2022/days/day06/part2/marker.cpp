#include "marker.hpp"
#include <cstdint>
#include <exception>
#include <unordered_map>
#include <iostream>

marker::marker(const std::string &line) {
	this->_data = line;
	this->_hash_map = std::unordered_map<char, uint32_t>(26);
	// init the hash map with lower case characters
	for (int i = 0; i < 26; ++i) {
		this->_hash_map.insert(std::pair<char, uint32_t>('a' + i, 0));
	}
	// init the hash map with the first 13 characters
	for (int i = 0; i < 13; ++i) {
		this->_hash_map[this->_data[i]]++;
	}
}

uint32_t
marker::find_mark() {
	for (int i = 13; i < this->_data.size(); ++i) {
		// Insert
		this->_hash_map[this->_data[i]]++;
		// Check for no dups
		bool has_found_dup = false;
		//std::cout << "------------------------------------" << std::endl;
		for (int j = 0; j < 26; ++j) {
			/*std::cout << "[" << char('a' + j) << "," << this->_hash_map['a' + j]
				<< "]" << std::endl;
				*/
			if (this->_hash_map['a' + j] > 1) {
				// Duplicate found, keep searching
				has_found_dup = true;
				//break;
			}
		}
		
		if (!has_found_dup) {
			return i + 1;
		}

		// Remove at front
		this->_hash_map[this->_data[i - 13]]--;
	}

	throw new std::exception();
}
