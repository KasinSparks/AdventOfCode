#ifndef AOC_TREE_HOUSE_HPP
#define AOC_TREE_HOUSE_HPP

#include "aoc_grid.hpp"
#include <cstdint>

struct aoc_cord {
	uint32_t x;
	uint32_t y;
} typedef aoc_cord;

enum aoc_direction {
	NORTH = 0001,
	SOUTH = 0010,
	EAST  = 0100,
	WEST  = 1000,
} typedef aoc_direction;

class tree_house : public aoc_grid<uint32_t> {
private:
public:
	tree_house(const uint32_t &width, const uint32_t &height);

	bool is_visible_from_edge(const uint32_t &x, const uint32_t &y, const aoc_direction &direction, const uint32_t &check_val);
};

#endif
