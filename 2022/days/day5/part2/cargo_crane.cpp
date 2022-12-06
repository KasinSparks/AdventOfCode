#include "cargo_crane.hpp"
#include <cstdint>
#include <stack>


cargo_crane::cargo_crane(const struct cargo_movement_data &data) {
	this->_cargo_data = data;
	this->_num_of_stacks = 0;
	this->_cargo_stacks = nullptr;
	this->_move_transforms = nullptr;
	this->_num_of_move_transforms = 0;

	this->parse_starting_config(data);
	this->parse_moving_orders(data);

	this->move();
}

cargo_crane::~cargo_crane() {
	if (this->_cargo_stacks != nullptr) {
		delete [] this->_cargo_stacks;
		this->_cargo_stacks = nullptr;
	}
	this->_num_of_stacks = 0;

	if (this->_move_transforms != nullptr) {
		delete [] this->_move_transforms;
		this->_move_transforms = nullptr;
	}
	this->_num_of_move_transforms = 0;

}

std::string
cargo_crane::get_top_items() const {}

void
cargo_crane::print() const {
	std::cout << "STACKS: Top of stack is on the left side" << std::endl;
	std::stack<char> *temp_stack = nullptr;
	if (this->_num_of_stacks > 0) {
		temp_stack = new std::stack<char>[this->_num_of_stacks];
	}

	for (uint32_t i = 0; i < this->_num_of_stacks; ++i) {
		temp_stack[i] = std::stack<char>(this->_cargo_stacks[i]);
	}



	for (uint32_t i = 0; i < this->_num_of_stacks; ++i) {
		std::cout << i + 1 << ": ";
		while (!temp_stack[i].empty()) {
			std::cout << temp_stack[i].top();
			temp_stack[i].pop();
		}
		std::cout << std::endl;
	}
	
	/*
	std::cout << "MOVE TRANSFORMS: " << std::endl;
	for (int i = 0; i < this->_num_of_move_transforms; ++i) {
		std::cout << this->_move_transforms[i].quantity
			<< ", " << this->_move_transforms[i].from_loc
			<< ", " << this->_move_transforms[i].to_loc << std::endl;
	}
	*/

	if (temp_stack != nullptr) {
		delete [] temp_stack;
	}
}
