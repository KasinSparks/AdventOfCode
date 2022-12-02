#include "file_reader.hpp"
#include <string>

std::list<std::string> read_file(const std::string &file) {
	std::ifstream fs;
	fs.open(file.c_str(), std::ifstream::in);
	std::list<std::string> string_list;
	
	std::string test;
	while(std::getline(fs, test)) {
		string_list.push_back(test);
		test = "";
	}

	fs.close();

	return string_list;
}

void
read_file(std::list<std::string> &str_list, const std::string &file) {
	std::ifstream fs;
	fs.open(file.c_str(), std::ifstream::in);
	
	std::string test;
	while(std::getline(fs, test)) {
		str_list.push_back(test);
		test = "";
	}

	fs.close();
}
