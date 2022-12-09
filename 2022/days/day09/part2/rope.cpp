#include "rope.hpp"
#include <cstdint>
#include <unordered_map>
#include <iostream>

rope::rope() {
	this->_tail_visited = std::unordered_map<aoc_point, uint64_t>();
}

void
rope::add_movement(const aoc_direction direction, const uint64_t &amount) {
	for (int i = 0; i < amount; ++i) {
		this->move(direction);
	}
}

uint64_t
rope::get_num_of_tail_visited() const {
	return this->_tail_visited.size();
}

void
rope::print_hashtable() const {
	for (std::pair<aoc_point, uint64_t> p : this->_tail_visited) {
		std::cout << p.first.to_string() << " : " << p.second << std::endl;
	}
}
