#include "directory.hpp"
#include <cstdint>
#include <iostream>

aoc_directory::aoc_directory(const std::string &dir_name, aoc_directory *parent) {
	this->_dir_name = dir_name;
	this->_parent = parent;
}

aoc_directory::aoc_directory(const aoc_directory &d) {
	this->_dir_name = d.get_name();
	this->_parent = d.get_parent();
	this->_sub_dirs = d._sub_dirs;
	this->_files = d._files;
}

uint64_t
aoc_directory::calculate_total_size() const {
	uint64_t total_size = 0;
	// Calculate the files sizes in the current directory
	// Do not include root directory files
	for (std::pair<std::string, aoc_file> f: this->_files) {
		total_size += f.second.get_size();
	}
	// Calculate the total space the sub-directories are holding
	for (std::pair<std::string, aoc_directory> dir : this->_sub_dirs) {
		total_size += dir.second.calculate_total_size();
	}

	return total_size;
}

uint64_t
aoc_directory::calculate_total_with_dups() const {
	uint64_t total_size = this->calculate_total_size();
	// Calculate the files sizes in the current directory
	// Do not include root directory files

	if (total_size > 100000) {
		total_size = 0;
	}
	
	std::cout << "In dir: " << this->_dir_name << std::endl;

	// Calculate the total space the sub-directories are holding
	for (std::pair<std::string, aoc_directory> dir : this->_sub_dirs) {
		std::cout << "Going into dir: " << dir.first << std::endl;
		// Only take ones with a size <= 100,000
		uint64_t temp_size = dir.second.calculate_total_size();
		if (temp_size <= 100000) {
			total_size += dir.second.calculate_total_with_dups_helper(1);	
		} else {
			total_size += dir.second.calculate_total_with_dups();
		}
		//total_size += dir.second.calculate_total_size();
		std::cout << "tv: " << total_size << std::endl;
	}


	return total_size;
}

uint64_t
aoc_directory::calculate_total_with_dups_helper(const uint32_t &depth) const {
	uint64_t total_size = this->calculate_total_size();
	// Calculate the files sizes in the current directory
	// Do not include root directory files
	
	std::cout << "In dir: " << this->_dir_name << std::endl;

	// Calculate the total space the sub-directories are holding
	for (std::pair<std::string, aoc_directory> dir : this->_sub_dirs) {
		std::cout << "Going into dir: " << dir.first << std::endl;
		// Only take ones with a size <= 100,000
		total_size += dir.second.calculate_total_with_dups_helper(depth);
		std::cout << "size of " << dir.first << ": " << total_size << std::endl;
		//total_size += dir.second.calculate_total_size();
	}
	
	std::cout << "size of " << this->_dir_name << " (with depth: " << depth << ") : " << total_size * depth << std::endl;
	return total_size * depth;
}

void
aoc_directory::add_dir(const aoc_directory &dir) {
	this->_sub_dirs.insert(std::pair<std::string, aoc_directory>(dir.get_name(), dir));
}

void
aoc_directory::add_file(const aoc_file &file) {
	this->_files.insert(std::pair<std::string, aoc_file>(file.get_filename(), file));
}

std::string
aoc_directory::get_name() const {
	return this->_dir_name;
}

aoc_directory *
aoc_directory::get_parent() const {
	return this->_parent;
}

aoc_directory *
aoc_directory::get_directory(const std::string dir_name) {
	return &(this->_sub_dirs.find(dir_name)->second);
}
