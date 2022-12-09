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
#define ROPE_LEN 10

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
	aoc_point _locations[ROPE_LEN];

	uint64_t get_distance(const aoc_point &p0, const aoc_point &p1) const {
		int64_t x = p0.get_x() - p1.get_x();
		int64_t y = p0.get_y() - p1.get_y();

		return ((x * x) + (y * y));
	}

	aoc_point get_closest_point(const aoc_point &p0, const aoc_point &p1) const {
		aoc_point modifier;

		if (p0.get_y() > p1.get_y()) {
			modifier.set_y(1);
		} else if (p0.get_y() < p1.get_y()) {
			modifier.set_y(-1);
		}

		if (p0.get_x() > p1.get_x()) {
			modifier.set_x(1);
		} else if (p0.get_x() < p1.get_x()) {
			modifier.set_x(-1);
		}

		return modifier;
	}

	void move(const aoc_direction direction) {
		// Store the last positions for later
		/*
		aoc_point last_pos[ROPE_LEN];
		for (int i = 0; i < ROPE_LEN; ++i) {
			last_pos[i] = this->_locations[i];
		}
		*/

		// Move the points
		switch (direction) {
			// Set the new head position
			case aoc_direction::NORTH:
				this->_locations[0].set_y(this->_locations[0].get_y() + 1);
				break;
			case aoc_direction::SOUTH:
				this->_locations[0].set_y(this->_locations[0].get_y() - 1);
				break;
			case aoc_direction::EAST:
				this->_locations[0].set_x(this->_locations[0].get_x() + 1);
				break;
			case aoc_direction::WEST:
				this->_locations[0].set_x(this->_locations[0].get_x() - 1);
				break;
		}
		
		// Pull the chain
		for (int i = 1; i < ROPE_LEN; ++i) {
			if (this->get_distance(this->_locations[i - 1], this->_locations[i]) > AOC_ROPE_MAX_DISTANCE) {
				// PULL
				aoc_point n_point = this->get_closest_point(this->_locations[i - 1], this->_locations[i]);
				this->_locations[i] += n_point;
			} else {
				// Done pulling
				break;
			}

			//std::cout << "(" << this->_locations[i].to_string() << "), ";
		}

		//std::cout << std::endl;


		/*
		std::cout << "HEAD: (" << this->_head_location.to_string() << ")";
		std::cout << ", TAIL: (" << this->_tail_location.to_string() << ")";
		std::cout << std::endl;
		*/

		// Store the tail's position
		this->_tail_visited[this->_locations[ROPE_LEN - 1]]++;
	}
public:
	rope();

	void add_movement(const aoc_direction direction, const uint64_t &amount);
	uint64_t get_num_of_tail_visited() const;

	void print_hashtable() const;
};


#endif
