#include "aoc_point.hpp"
#include <string>

aoc_point::aoc_point() {
	this->_x = 0;
	this->_y = 0;
}

aoc_point::aoc_point(const int64_t &x, const int64_t &y) {
	this->_x = x;
	this->_y = y;
}

aoc_point::aoc_point(const aoc_point &p) {
	this->_x = p.get_x();
	this->_y = p.get_y();
}

int64_t
aoc_point::get_x() const {
	return this->_x;
}

int64_t
aoc_point::get_y() const {
	return this->_y;
}

void
aoc_point::set_x(const int64_t &x) {
	this->_x = x;
}

void
aoc_point::set_y(const int64_t &y) {
	this->_y = y;
}

void
aoc_point::set_point(const aoc_point &p) {
	this->_x = p.get_x();
	this->_y = p.get_y();
}

bool
aoc_point::operator== (const aoc_point &p) const {
	return (this->_x == p.get_x() && this->_y == p.get_y());
}

bool
aoc_point::operator!= (const aoc_point &p) const {
	return (this->_x != p.get_x() || this->_y != p.get_y());
}

aoc_point
aoc_point::operator+(const aoc_point &p) const {
	aoc_point n_point;
	n_point.set_x(this->_x + p.get_x());
	n_point.set_y(this->_y + p.get_y());

	return n_point;
}

void
aoc_point::operator+=(const aoc_point &p) {
	this->_x += p.get_x();
	this->_y += p.get_y();
}

std::string
aoc_point::to_string() const {
	return std::to_string(this->_x) + "," + std::to_string(this->_y);
}
