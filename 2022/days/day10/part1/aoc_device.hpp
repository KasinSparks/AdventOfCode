#ifndef AOC_DEVICE_HPP
#define AOC_DEVICE_HPP

#include <cstdint>
#include <string>
#include <iostream>

class aoc_device {
private:
	uint64_t _cycle_count;
	int64_t  _x_register;

	bool _is_ready_to_report;
	int64_t _cycle_report_val;
	int64_t _cycle_report_it;
	int64_t _cycle_report_reg;
protected:
	void check_report_status() {
		if (this->_cycle_count == this->_cycle_report_val) {
			this->_is_ready_to_report = true;
			this->_cycle_report_reg = this->_x_register;
			this->_cycle_report_it = this->_cycle_count;
			
			this->_cycle_report_val += 40;
		}
	}

	void add(const int64_t &value) {
		for (int i = 0; i < 2; ++i) {
			this->_cycle_count++;
			this->check_report_status();
		}
		this->_x_register += value;
	}

	void noop() {
		this->_cycle_count++;
		this->check_report_status();
	}
public:
	aoc_device();

	void parse(const std::string &data);
	int64_t get_cycle_count() const;
	int64_t get_signal_strength() const;
	bool get_report_status();
	void set_report_status_false();
};

#endif
