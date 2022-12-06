#ifndef AOC_CARGO_CRANE_HPP
#define AOC_CARGO_CRANE_HPP

#include <cstdint>
#include <exception>
#include <stack>
#include <string>
#include <list>
#include <iostream>
#include <utility>

struct cargo_movement_data {
	std::string *lines;
	uint32_t     first_data_stop;
	uint32_t     lines_size;
};

struct cargo_transform {
	uint32_t quantity;
	uint32_t from_loc;
	uint32_t to_loc;
};

class cargo_crane {
private:
	/// A place to store our stacks
	std::stack<char> *_cargo_stacks;

	/// Count of stacks
	uint32_t _num_of_stacks;
	
	/// Simple input data
	struct cargo_movement_data _cargo_data;
	
	/// Move transform data
	struct cargo_transform *_move_transforms = nullptr;
	uint32_t _num_of_move_transforms = 0;

protected:
	void parse_starting_config(const struct cargo_movement_data &data) {
		// Get the number of stacks we need
		std::string num_info = data.lines[data.first_data_stop];
		// Parse the info
		// <number, column>
		std::list<std::pair<uint32_t, uint32_t>> nums;
		uint32_t column = 0;
		for (char c : data.lines[data.first_data_stop - 1]) {
			// Check for space or data
			if (c != ' ' && c != '\n') {
				// Data. Check for number
				if (!(c >= '0' && c <= '9')) {
					std::cerr << "Parse staring config: Check for number FAILED"
						<< std::endl;
					throw new std::exception();
				}
				// Put the number in the list
				nums.push_back(std::pair<uint32_t, uint32_t>(c - '0', column));
			}
			
			// Increase the column counter
			column++;
		}
		// Get the highest number
		uint32_t highest = 0;
		for (std::pair<uint32_t, uint32_t> p : nums) {
			//std::cout << "<" << p.first << ", " << p.second << ">" << std::endl;
			if (p.first > highest) {
				highest = p.first;
			}
		}
		// Create the stacks
		this->_num_of_stacks = highest;
		this->_cargo_stacks = new std::stack<char>[this->_num_of_stacks];

		// Add the data to the stacks from the bottom up
		for (int i = data.first_data_stop - 2; i >= 0; --i) {
			for (std::pair<uint32_t, uint32_t> p : nums) {
				if (data.lines[i][p.second] == ' ') {
					continue;
				}
				//std::cout << "pushing: " << data.lines[i][p.second] << std::endl;
				this->_cargo_stacks[p.first - 1].push(data.lines[i][p.second]);
			}
		}
	}

	void parse_moving_orders(const struct cargo_movement_data &data) {
		// The data is in the form of: move XX from XX to XX
		// where XX is a unknown digit number
		// Determine the number of lines to Parse
		uint32_t num_of_parse_lines = data.lines_size - data.first_data_stop;
		// Get the <Quantity, FromLocation, ToLocation>
		this->_num_of_move_transforms = num_of_parse_lines - 1;	
		this->_move_transforms = new struct cargo_transform[this->_num_of_move_transforms];
		// Init all the structs
		for (int i = 0; i < this->_num_of_move_transforms; ++i) {
			this->_move_transforms[i] = cargo_transform();
		}

		std::cout << num_of_parse_lines << std::endl;
		for (int i = data.first_data_stop + 1; i < data.lines_size; ++i) {
			// Parse the line
			// Find strings surrounded by a space
			uint32_t p[] = {0, 0, 0, 0, 0};
			uint32_t p_counter = 0;
			for (int j = 0; j < data.lines[i].size(); ++j) {
				if (data.lines[i][j] == ' ') {
					p[p_counter++] = j;
				}
			}

			this->_move_transforms[i - (data.first_data_stop + 1)] = cargo_transform {
				uint32_t(std::stoi(data.lines[i].substr(p[0], p[1] - p[0]))),
				uint32_t(std::stoi(data.lines[i].substr(p[2], p[3] - p[2]))),
				uint32_t(std::stoi(data.lines[i].substr(p[4])))
			};
			
			/*
			std::cout << data.lines[i].substr(p[0], p[1] - p[0]) << ", "
				<< data.lines[i].substr(p[2], p[3] - p[2]) << ", "
				<< data.lines[i].substr(p[4]) << std::endl;
			*/
		}
	}

	void move() {
		// Move the amount from one stack onto another
		for (int i = 0; i < this->_num_of_move_transforms; ++i) {
			// Start moving data
			struct cargo_transform &ct = this->_move_transforms[i];
			for (int j = 0; j < this->_move_transforms[i].quantity; ++j) {
				char temp_c = this->_cargo_stacks[ct.from_loc - 1].top();
				this->_cargo_stacks[ct.to_loc - 1].push(temp_c);
				this->_cargo_stacks[ct.from_loc - 1].pop();
			}

			this->print();
		}
	}
public:
	cargo_crane(const struct cargo_movement_data &data);
	~cargo_crane();	
	
	/// Returns a sequence of items on the top of each stack
	std::string get_top_items() const;

	/// Print the data to std out
	void print() const;
};

#endif
