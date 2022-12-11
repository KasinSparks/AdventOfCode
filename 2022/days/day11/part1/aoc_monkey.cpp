#include "aoc_monkey.hpp"
#include <cstdint>
#include <exception>

aoc_monkey::aoc_monkey() {
	this->_monkey_number = 0;
	this->_items = std::list<uint32_t>();
	this->_op = aoc_monkey_operation::UNK;
	this->_test_val = 0;
	this->_monkey_val_true = 0;
	this->_monkey_val_false = 0;
	this->_num_of_inspected_items = 0;
}

void
aoc_monkey::parse_input_block(const std::string *input_block) {
	// Monkey Line
	std::string monkey_num = input_block[0].substr(6, (input_block[0].length() - 6) - 1);
	this->_monkey_number = std::stoi(monkey_num);

	// Starting Line
	std::string item_list = input_block[1].substr(18);
	// Split the csv
	std::string temp = "";
	for (char c : item_list) {
		if (c == ',') {
			this->_items.push_back(std::stoi(temp));
			temp = "";
		} else if (c == ' ') {
			continue;
		} else {
			temp.push_back(c);
		}
	}

	if (!temp.empty()) {
		this->_items.push_back(std::stoi(temp));
	}

	// Operation
	std::string operation = input_block[2].substr(23);
	if (operation.substr(0, 1) == "*") {
		// Multiply
		if (operation.substr(2, 1) == "o") {
			// old^2
			this->_op = aoc_monkey_operation::MULTIPY_OLD;
		} else {
			// Regular multiply
			this->_op = aoc_monkey_operation::MULTIPY;
			this->_op_val = std::stoi(operation.substr(2));
		}
	} else {
		// Addition
		this->_op = aoc_monkey_operation::ADD;
		this->_op_val = std::stoi(operation.substr(2));
	}

	// Test
	this->_test_val = std::stoi(input_block[3].substr(21));

	// If true
	this->_monkey_val_true = std::stoi(input_block[4].substr(29));

	// If false 
	this->_monkey_val_false = std::stoi(input_block[5].substr(30));	
}

void
aoc_monkey::print() const {
	std::cout << "Monkey " << this->_monkey_number << ":" << std::endl;
	// Starting items
	std::cout << "  Starting items: ";
	for (uint32_t item : this->_items) {
		std::cout << item << ", ";	
	}
	std::cout << std::endl;
	// Operation
	std::cout << "  Operation: new = old";
	switch (this->_op) {
		case aoc_monkey_operation::ADD:
			std::cout << " + " << this->_op_val;
			break;
		case aoc_monkey_operation::MULTIPY:
			std::cout << " * " << this->_op_val;
			break;
		case aoc_monkey_operation::MULTIPY_OLD:
			std::cout << " * old";
			break;
		default:
			std::cout << "UNKNOWN";
			break;
	}
	std::cout << std::endl;
	// Test
	std::cout << "  Test: divisible by " << this->_test_val << std::endl;
	std::cout << "    If true: throw to monkey " << this->_monkey_val_true
		<< std::endl;
	std::cout << "    If false: throw to monkey " << this->_monkey_val_false
		<< std::endl;
}

void
aoc_monkey::run(std::list<aoc_monkey_thrown_item> *thrown_items) {
	uint32_t new_worry_level = 0;

	while (!this->_items.empty()) {
		this->_num_of_inspected_items++;
		uint32_t item = this->_items.front();
		// Inspect item
		switch (this->_op) {
			case aoc_monkey_operation::ADD:
				new_worry_level = item + this->_op_val;
				break;
			case aoc_monkey_operation::MULTIPY:
				new_worry_level = item * this->_op_val;
				break;
			case aoc_monkey_operation::MULTIPY_OLD:
				new_worry_level = item * item;
				break;
			default:
				std::cerr << "Unknown operation" << std::endl;
				throw new std::exception();
				break;
		}
		
		new_worry_level = new_worry_level / 3;

		aoc_monkey_thrown_item amti;
		//new_worry_level = new_worry_level / this->_test_val;
		if (new_worry_level % this->_test_val == 0) {
			amti = aoc_monkey_thrown_item {
						this->_monkey_val_true,
						new_worry_level
					};

		} else {
			amti = aoc_monkey_thrown_item {
						this->_monkey_val_false,
						new_worry_level
					};
		}
		thrown_items->push_back(amti);
		this->_items.pop_front();
	}
}

void
aoc_monkey::add(const uint32_t &item_id) {
	this->_items.push_back(item_id);
}

uint32_t
aoc_monkey::get_num_inspected() const {
	return this->_num_of_inspected_items;
}
