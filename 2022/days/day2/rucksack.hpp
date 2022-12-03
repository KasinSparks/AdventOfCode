#ifndef AOC_RUCKSACK_HPP
#define AOC_RUCKSACK_HPP

#include <string>

class rucksack {
private:
	char _item;
public:
	rucksack(const std::string &line);

	uint32_t get_duplicate_item_priority();
};

#endif
