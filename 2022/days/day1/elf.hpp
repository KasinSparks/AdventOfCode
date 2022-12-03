#ifndef AOC_ELF_HPP
#define AOC_ELF_HPP

#include <list>
#include <sys/types.h>

class elf {

private:
	std::list<u_int32_t> *_food;
public:
	elf();
	~elf();
	void add_food(u_int32_t calories);
	u_int32_t get_total_calories() const;

	bool operator < (const elf &e0) const {
		return this->get_total_calories() < e0.get_total_calories();
	}

	bool operator == (const elf &e0) const {
		return this->get_total_calories() == e0.get_total_calories();
	}
};

#endif
