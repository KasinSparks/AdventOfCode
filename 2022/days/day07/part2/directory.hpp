#ifndef AOC_DIRECTORY_HPP
#define AOC_DIRECTORY_HPP

#include <cstdint>
#include <unordered_map>

#include "files.hpp"

class aoc_directory {
private:
	std::string _dir_name;
	aoc_directory *_parent;
	std::unordered_map<std::string, aoc_directory> _sub_dirs;
	std::unordered_map<std::string, aoc_file>      _files;
public:
	aoc_directory(const std::string &dir_name, aoc_directory *parent);
	aoc_directory(const aoc_directory &d);

	uint64_t calculate_total_size() const;
	uint64_t calculate_total_with_dups() const;
	uint64_t calculate_total_with_dups_helper(const uint32_t &depth) const;
	void add_dir(const aoc_directory &dir);
	void add_file(const aoc_file &file);

	aoc_directory *get_directory(const std::string dir_name);

	std::string get_name() const;
	aoc_directory *get_parent() const;

	const aoc_directory *get_directory_to_delete(const uint64_t &needed_space);
};

#endif
