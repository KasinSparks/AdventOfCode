#ifndef AOC_DEVICE_HPP
#define AOC_DEVICE_HPP

#include <cstdint>
#include <string>
#include <iostream>

class aoc_device {
private:
	uint64_t _cycle_count;
	int64_t  _x_register;
protected:
	void add(const int64_t &value) {
		for (int i = 0; i < 2; ++i) {
			this->cycle();
		}
		this->_x_register += value;
	}

	void noop() {
		this->cycle();
	}

	void cycle() {
		this->print_crt();
		this->_cycle_count++;
	}

	void print_crt() const {
		uint64_t pos = this->_cycle_count % 40;
		if (this->_cycle_count != 0 && pos == 0) {
			std::cout << std::endl;
		}

		if (pos >= this->_x_register - 1 && pos <= this->_x_register + 1) {
			std::cout << "#";
		} else {
			std::cout << ".";
		}
	}
public:
	aoc_device();

	void parse(const std::string &data);
	int64_t get_cycle_count() const;
};

#endif
