#include "aoc_device.hpp"
#include <cstdint>
#include <iostream>

aoc_device::aoc_device() {
	this->_cycle_count = 0;
	this->_x_register = 1;
}

void
aoc_device::parse(const std::string &data) {
	std::string instruction = data.substr(0, 4);

	if (instruction == "noop") {
		this->noop();
	} else if (instruction == "addx") {
		int64_t value = std::stoi(data.substr(5));
		this->add(value);
	}
}

int64_t
aoc_device::get_cycle_count() const {
	return this->_cycle_count;
}
