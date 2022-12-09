#include "files.hpp"

aoc_file::aoc_file(const uint32_t &size, const std::string &filename) {
	this->_size = size;
	this->_filename = filename;
}

aoc_file::aoc_file(const aoc_file &f) {
	this->_size = f.get_size();
	this->_filename = f.get_filename();
}

uint32_t
aoc_file::get_size() const {
	return this->_size;
}

std::string
aoc_file::get_filename() const {
	return this->_filename;
}
