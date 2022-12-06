#ifndef AOC_MARKER_HPP
#define AOC_MARKER_HPP

#include <exception>
#include <string>
#include <queue>
#include <unordered_map>

class marker {
private:
	std::string _data;
	std::unordered_map<char, uint32_t> _hash_map;
public:
	marker(const std::string &line);
	
	/// Returns the location of the marker
	uint32_t find_mark();
};

#endif
