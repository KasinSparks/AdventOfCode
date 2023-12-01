#ifndef AOC_POINT_HPP
#define AOC_POINT_HPP

#include <cstdint>
#include <string>

class aoc_point {
private:
	int64_t _x;
	int64_t _y;
public:
	aoc_point();
	aoc_point(const int64_t &x, const int64_t &y);
	aoc_point(const aoc_point &p);

	int64_t get_x() const;
	int64_t get_y() const;
	void set_x(const int64_t &x);
	void set_y(const int64_t &y);
	void set_point(const aoc_point &p);

	bool operator== (const aoc_point &p) const;
	bool operator!= (const aoc_point &p) const;
	aoc_point operator+(const aoc_point &p) const;
	void operator+=(const aoc_point &p);
	bool operator< (const aoc_point &p) const;

	std::string to_string() const;
};

template<> struct std::hash<aoc_point> {
	std::size_t operator()(const aoc_point &p) const {
		return std::hash<std::string>{}(p.to_string());
	}
};


#endif
