#include "aoc_device.hpp"
#include <cstdint>
#include <iostream>

aoc_device::aoc_device() {
	this->_cycle_count = 0;
	this->_x_register = 1;

	this->_is_ready_to_report = false;
	this->_cycle_report_it = 0;
	this->_cycle_report_reg = 0;
	this->_cycle_report_val = 20;
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

int64_t
aoc_device::get_signal_strength() const {
	std::cout << this->_cycle_report_it << " * " << this->_cycle_report_reg
		<< " = " << this->_cycle_report_it * this->_cycle_report_reg << std::endl;
	return this->_cycle_report_it * this->_cycle_report_reg;

}

bool
aoc_device::get_report_status() {
	return this->_is_ready_to_report;
}

void
aoc_device::set_report_status_false() {
	this->_cycle_report_reg = 0;
	this->_is_ready_to_report = false;
}
