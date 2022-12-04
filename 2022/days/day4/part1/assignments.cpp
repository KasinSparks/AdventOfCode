#include "assignments.hpp"
#include <cstdint>

assignment::assignment() {
	this->_beginning = 0;
	this->_beginning = 0;
}

assignment::assignment(const assignment &a) {
	this->_beginning = a.get_beginning();
	this->_ending = a.get_ending();
}

assignment::assignment(const uint32_t b, const uint32_t e) {
	this->_beginning = b;
	this->_ending = e;
}

uint32_t
assignment::get_beginning() const { return this->_beginning; }

uint32_t
assignment::get_ending() const { return this->_ending; }


bool
assignment::fully_contains(const assignment &a) {
	return this->_beginning <= a.get_beginning() && this->_ending >= a.get_ending();
}
