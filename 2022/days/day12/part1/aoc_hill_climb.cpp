#include "aoc_hill_climb.hpp"
#include <cstdint>
#include <unordered_map>

aoc_hill_climb::aoc_hill_climb(const std::list<std::string> &data) {
	this->_grid = nullptr;
	this->parse(data);
}

aoc_hill_climb::~aoc_hill_climb() {
	if (this->_grid != nullptr) {
		delete this->_grid;
	}
}

uint32_t
aoc_hill_climb::climb() {
	std::unordered_map<aoc_point, bool> visited_points;
	for (int i = 0; i < this->_grid->get_height(); ++i) {
		for (int j = 0; j < this->_grid->get_width(); ++j) {
			//std::cout << "i: " << i << ", j: " << j << std::endl;
			aoc_point p(j, i);
			visited_points[p] = false;
		}

	}
	//return this->depth_first_search(aoc_point(0, 0), visited_points);
	return this->depth_first_search(aoc_point(0, 0), visited_points);
}
