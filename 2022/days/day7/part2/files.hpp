#ifndef AOC_FILE_HPP
#define AOC_FILE_HPP

#include <cstdint>
#include <string>

class aoc_file {
private:
	uint32_t    _size;
	std::string _filename;
public:
	aoc_file(const uint32_t &size, const std::string &filename);
	aoc_file(const aoc_file &f);
	uint32_t get_size() const;
	std::string get_filename() const;
};

#endif
