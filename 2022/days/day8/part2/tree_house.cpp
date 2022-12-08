#include "tree_house.hpp"
#include <cstdint>


tree_house::tree_house(const uint32_t &width, const uint32_t &height) :
	aoc_grid<uint32_t>(width, height) {}

bool
tree_house::is_visible_from_edge(const uint32_t &x, const uint32_t &y, const aoc_direction &direction, const uint32_t &check_val, uint32_t &depth) {
	if (!this->check_bound(x, y, false)) {
		// Reached past and edge
		return true;
	}
	// Go out to each edge testing if the value is less than the current	
	uint32_t neighborhood[9];
	this->get_neighbors(x, y, neighborhood);
	
	// Test the points around the current point
	bool is_visible = false;
	aoc_cord cords = {0, 0};
	aoc_cord check_cord = {0, 0};
	
	switch (direction) {
		case aoc_direction::NORTH:
			cords = {x, y - 1};
			check_cord = {1, 0};
			break;
		case aoc_direction::SOUTH:
			cords = {x, y + 1};
			check_cord = {1, 2};
			break;
		case aoc_direction::EAST:
			cords = {x + 1, y};
			check_cord = {2, 1};
			break;
		case aoc_direction::WEST:
			cords = {x - 1, y};
			check_cord = {0, 1};
			break;
	}
	
	uint32_t neighbor_val = neighborhood[check_cord.x + (check_cord.y * 3)];
	if (neighbor_val == -1) {
		return true;
	} else if (neighbor_val >= check_val) {
		++depth;
		return false;
	};

	is_visible |= this->is_visible_from_edge(cords.x, cords.y, direction, check_val, ++depth);
	
	return is_visible;
}
