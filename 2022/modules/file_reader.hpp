#ifndef AOC_FILE_READER_HPP
#define AOC_FILE_READER_HPP

#include <stdint.h>
#include <string>
#include <iostream>
#include <fstream>
#include <list>

/// Read a file's contents into memory.
/// Memory allocated needs to be freed
std::list<std::string> read_file(const std::string &file);

#endif
