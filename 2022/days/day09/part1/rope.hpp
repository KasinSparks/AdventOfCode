#ifndef AOC_ROPE_HPP
#define AOC_ROPE_HPP

#include "../../../modules/aoc_point.hpp"
//#include <list>
#include <functional>
#include <unordered_map>
#include <cstdint>
#include <string>
#include <iostream>

#define AOC_ROPE_MAX_DISTANCE 2

enum aoc_direction {
	NORTH = 1,
	SOUTH = 2,
	EAST  = 4,
	WEST  = 8,
} typedef aoc_direction;

template<> struct std::hash<aoc_point> {
	std::size_t operator()(const aoc_point &p) const {
		return std::hash<std::string>{}(p.to_string());
	}
};

class rope {
private:
	//aoc_grid<uint8_t> _tail_visited;
	//std::list<aoc_point> _tail_visited;
	std::unordered_map<aoc_point, uint64_t> _tail_visited;
	aoc_point _head_location;
	aoc_point _tail_location;

	uint64_t get_distance() const {
		int64_t x = this->_head_location.get_x() - this->_tail_location.get_x();
		int64_t y = this->_head_location.get_y() - this->_tail_location.get_y();

		return ((x * x) + (y * y));
	}

	void move(const aoc_direction direction) {
		// Store the head's last position for later
		aoc_point head_last_pos = this->_head_location;

		// Move the points
		switch (direction) {
			case aoc_direction::NORTH:
				// Set the new head position
				this->_head_location.set_y(this->_head_location.get_y() + 1);
				break;
			case aoc_direction::SOUTH:
				// Set the new head position
				this->_head_location.set_y(this->_head_location.get_y() - 1);
				break;
			case aoc_direction::EAST:
				// Set the new head position
				this->_head_location.set_x(this->_head_location.get_x() + 1);
				break;
			case aoc_direction::WEST:
				// Set the new head position
				this->_head_location.set_x(this->_head_location.get_x() - 1);
				break;
		}
		
		if (this->get_distance() > AOC_ROPE_MAX_DISTANCE) {
			this->_tail_location.set_point(head_last_pos);
		}


		/*
		std::cout << "HEAD: (" << this->_head_location.to_string() << ")";
		std::cout << ", TAIL: (" << this->_tail_location.to_string() << ")";
		std::cout << std::endl;
		*/

		// Store the tail's position
		this->_tail_visited[this->_tail_location]++;
	}
public:
	rope();

	void add_movement(const aoc_direction direction, const uint64_t &amount);
	uint64_t get_num_of_tail_visited() const;

	void print_hashtable() const;
};


#endif
