#ifndef AOC_HILL_CLIMB
#define AOC_HILL_CLIMB

#include "../../../modules/aoc_grid.hpp"
#include "../../../modules/aoc_point.hpp"

#include <cstdint>
#include <list>
#include <unordered_map>

class aoc_hill_climb {
private:
	aoc_grid<char> *_grid;

	uint32_t depth_first_search(const aoc_point &point, std::unordered_map<aoc_point, bool> &visited_points) const {
		uint32_t total = 1;
		// Test if we reached the end point
		if (this->_grid->get(point) == 'E') {
			std::cout << "reached the end" << std::endl;
			return total;
		}
		
		const uint8_t num_of_neighbors = 9;
		char neighborhood[num_of_neighbors];
		this->_grid->get_neighbors(point, neighborhood);
		visited_points[point] = true;
		std::cout << "point: " << point.to_string() << std::endl;

		// Ignore the middle point [4] and and point who's value == -1
		// Get rid of the diagonals as well
		for (int i = 0; i < num_of_neighbors; ++i) {
			if ((i == 1 || i == 3 || i == 5 || i == 7) && neighborhood[i] != -1) {
				aoc_point neighbor_loc = aoc_point(point.get_x() + (i % 3) - 1, point.get_y() + (i / 3) - 1);
				std::cout << "visited_point (" << neighbor_loc.to_string() << "): " << visited_points[neighbor_loc] << std::endl;
				if (!visited_points[neighbor_loc]) {
					// Label the current edge as been visited
					//visited_points[std::pair<aoc_point, aoc_point>(point, neighbor_loc)] = true;
					std::cout << "testing: " << neighbor_loc.to_string() << std::endl;
					if (this->_grid->get(neighbor_loc) < this->_grid->get(point) + 1) {
						std::cout << neighbor_loc.to_string() << std::endl;
						//total += depth_first_search(neighbor_loc, visited_points);
						total += depth_first_search(neighbor_loc, visited_points);
					}
				}
			}
		}
		for (int i = 0; i < num_of_neighbors; ++i) {
			if ((i == 1 || i == 3 || i == 5 || i == 7) && neighborhood[i] != -1) {
				aoc_point neighbor_loc = aoc_point(point.get_x() + (i % 3) - 1, point.get_y() + (i / 3) - 1);
				visited_points[neighbor_loc] = false;
			}
		}


		return total;
	}

	uint32_t path_search(const aoc_point &point) const {
		
		// Do a depth-first-search
		


		// Add the path length to a list
		
		// Find the smallest length in the path length list

	}

	void parse(const std::list<std::string> &list) {
		this->_grid = new aoc_grid<char>(list.front().length(), list.size());
		
		int count = 0;
		for (std::string line : list) {
			for (int i = 0; i < line.length(); ++i) {
				this->_grid->set(i, count, line[i]);
			}
		}
	}
public:
	aoc_hill_climb(const std::list<std::string> &data);
	~aoc_hill_climb();

	uint32_t climb();
};

#endif
