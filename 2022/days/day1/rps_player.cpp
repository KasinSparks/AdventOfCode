#include "rps_player.hpp"


rps_player::rps_player() {}

uint16_t
rps_player::play_round(const std::string &line) const {
	// Check if the line is valid length
	if (line.size() != 3) {
		throw new std::exception();
	}

	// Split the line
	char opponet_hand = line[0];
	char players_hand = line[2];

	return this->clacluate_score(opponet_hand, players_hand);
}
