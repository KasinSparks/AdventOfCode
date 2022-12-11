#ifndef AOC_MONKEY_HPP
#define AOC_MONKEY_HPP

#include <list>
#include <cstdint>
#include <string>
#include <iostream>

enum aoc_monkey_operation {
	UNK,
	MULTIPY,
	ADD,
	MULTIPY_OLD,
} typedef aoc_monkey_operation;

struct aoc_monkey_thrown_item {
	uint32_t dest_monkey_id;
	uint32_t payload;
} typedef aoc_monkey_thrown_item;

class aoc_monkey {
private:
	uint32_t _monkey_number;
	std::list<uint32_t> _items;
	aoc_monkey_operation  _op;
	uint32_t _op_val;
	uint32_t _test_val;
	uint32_t _monkey_val_true;
	uint32_t _monkey_val_false;
	uint32_t _num_of_inspected_items;
public:
	aoc_monkey();
	
	/// Parse the block of input. The input block MUST be 6 lines.
	void parse_input_block(const std::string *input_block);
	
	/// Print the output to stdout as it is formated on the aoc website
	void print() const;

	/// Get a list of thrown items and run the simulation.
	void run(std::list<aoc_monkey_thrown_item> *thrown_items);

	/// Add a item to this monkey's queue
	void add(const uint32_t &item_id);

	/// Returns how many items this monkey has inspected
	uint32_t get_num_inspected() const;
};

#endif
