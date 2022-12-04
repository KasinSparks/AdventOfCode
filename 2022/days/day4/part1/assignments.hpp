#ifndef AOC_ASSIGNMENTS_HPP
#define AOC_ASSIGNMENTS_HPP

#include <cstdint>

class assignment {
private:
	uint32_t _beginning;
	uint32_t _ending;
public:
	assignment();
	assignment(const assignment &a);
	/// Takes a b: beginning and e: ending of range
	assignment(const uint32_t b, const uint32_t e);
	
	// Returns true if this assignment is able to fully contain given assignment
	bool fully_contains(const assignment &a);

	uint32_t get_beginning() const;
	uint32_t get_ending() const;
};

#endif
