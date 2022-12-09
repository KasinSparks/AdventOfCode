#ifndef AOC_RUCKSACK_HPP
#define AOC_RUCKSACK_HPP

#include <cstdint>
#include <string>
#include <unordered_map>

class rucksack {
private:
	std::unordered_map<char, uint8_t> _items;

	std::unordered_map<char, uint8_t> _remove_duplicates(const std::string &line) {
		std::unordered_map<char, uint8_t> seen;
		for (char c : line) {
			auto search = seen.find(c);
			if (search == seen.end()) {
				seen.insert(std::pair<char, uint8_t>(c, 1));
			}
		}

		return seen;
	}
public:
	rucksack();

	uint32_t get_item_priority();
	void add_line(const std::string &line);
};

#endif
