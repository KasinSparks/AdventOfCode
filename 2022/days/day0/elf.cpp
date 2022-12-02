#include "elf.hpp"
#include <iterator>
#include <sys/types.h>

elf::elf() {
	this->_food = new std::list<u_int32_t>();
}

elf::~elf() {
	delete this->_food;
	this->_food = nullptr;
}

void
elf::add_food(u_int32_t calories) {
	this->_food->push_back(calories);
}

u_int32_t
elf::get_total_calories() const {
	u_int32_t sum = 0;
	for (u_int32_t c : *(this->_food)) {
		sum += c;
	}
	return sum;
}

