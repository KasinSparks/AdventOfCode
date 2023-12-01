#ifndef AOC_GRID_HPP
#define AOC_GRID_HPP

#include "aoc_point.hpp"
#include <cstdint>
#include <exception>
#include <iostream>

template <typename T> 
class aoc_grid {
private:
	uint32_t _width;
	uint32_t _height;
	
	/// Data will be striped on width
	T *_data;
protected:
	/// Return true if the bound check passes
	bool check_bound(const uint32_t &x, const uint32_t &y, const bool &print) const {
		if (x > this->_width - 1 || y > this->_height - 1) {
			if (print) {
				std::cerr << "Out of bounds (" << x << "," << y << ") of grid<w:"
					<< this->_width << ",h:" << this->_height << ">" << std::endl;
			}
			return false;
		}

		return true;
	}
	
public:
	aoc_grid(const uint32_t &width, const uint32_t &height);
	~aoc_grid();
	
	/// 0 to size - 1
	void set(const uint32_t &x, const uint32_t &y, const T &data);
	const T get(const uint32_t &x, const uint32_t &y) const;
	const T get(const aoc_point &point) const;

	const uint32_t get_width() const;
	const uint32_t get_height() const;

	void print();

	/// If value is -1, then it is out of bounds
	void get_neighbors(const uint32_t &x, const uint32_t &y, T *neighbors) const;
	void get_neighbors(const aoc_point &point, T *neighbors) const;
	void get_neighbors(const aoc_point &point, aoc_point *neighbors) const;

};

template <typename T>
aoc_grid<T>::aoc_grid(const uint32_t &width, const uint32_t &height) {
	this->_width = width;
	this->_height = height;

	this->_data = new T[this->_width * this->_height];
}

template <typename T>
aoc_grid<T>::~aoc_grid() {
	if (this->_data != nullptr) {
		delete [] this->_data;
		this->_data = nullptr;
	}
}

template <typename T>
void
aoc_grid<T>::set(const uint32_t &x, const uint32_t &y, const T &data) {
	if (!this->check_bound(x, y, true)) {
		throw new std::exception();
	}

	this->_data[x + (this->_width * y)] = data;
}

template <typename T>
const T
aoc_grid<T>::get(const uint32_t &x, const uint32_t &y) const {
	if (!this->check_bound(x, y, false)) {
		throw new std::exception();
	}

	return this->_data[x + (this->_width * y)];
}

template <typename T>
void
aoc_grid<T>::print() {
	for (int i = 0; i < this->_height; ++i) {
		for (int j = 0; j < this->_width; ++j) {
			std::cout << this->_data[(i * this->_width) + j] << " ";
		}
		std::cout << std::endl;
	}
}

template <typename T>
const uint32_t
aoc_grid<T>::get_width() const {
	return this->_width;
}

template <typename T>
const uint32_t
aoc_grid<T>::get_height() const {
	return this->_height;
}

template <typename T> 
void
aoc_grid<T>::get_neighbors(const uint32_t &x, const uint32_t &y, T *neighbors) const {
	// Get the neighborhood
	for (int i = 0; i < 9; ++i) {
		try {
			neighbors[i] = this->get(x + (i % 3) - 1, y + (i / 3) - 1);
		} catch (std::exception *ex) {
			neighbors[i] = -1;	
			delete ex;
		}
	}

	/*
	for (int i = 1; i < 10; ++i) {
		std::cout << neighbors[i - 1] << ", ";
		if (i % 3 == 0) {
			std::cout << std::endl;
		}
	}
	std::cout << std::endl;
	*/
}

template<typename T>
void
aoc_grid<T>::get_neighbors(const aoc_point &point, T *neighbors) const {
	this->get_neighbors(point.get_x(), point.get_y(), neighbors);
}

template<typename T>
void
aoc_grid<T>::get_neighbors(const aoc_point &point, aoc_point *neighbors) const {
	for (int i = 0; i < 9; ++i) {
		neighbors[i] = aoc_point(point.get_x() + (i % 3) - 1, point.get_y() + (i / 3) - 1);
	}
}

template <typename T>
const T
aoc_grid<T>::get(const aoc_point &point) const {
	return this->get(point.get_x(), point.get_y());
}
#endif
