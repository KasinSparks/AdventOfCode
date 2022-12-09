#ifndef AOC_RPS_PLAYER_HPP
#define AOC_RPS_PLAYER_HPP

#include <cstdint>
#include <exception>
#include <string>

class rps_player {
private:
	uint8_t get_score(const char &hand) const {
		uint8_t score = 0;
		switch (hand) {
			case 'A': score = 1; break;
			case 'B': score = 2; break;
			case 'C': score = 3; break;
		}

		return score;
	}

	uint16_t clacluate_score(const char &opponet_hand, const char &players_hand) const {
		uint16_t score = 0;
		
		// Get the value from the hand the player showed
		switch (players_hand) {
			case 'X': { score += 0; break; }
			case 'Y': { score += 3; break; }
			case 'Z': { score += 6; break; }
			default: throw new std::exception();
		}


		// Determine the hand needed
		if (players_hand == 'Y') {
			// Draw
			score += this->get_score(opponet_hand);
		} else if (players_hand == 'X') {
			// Lose
			if (opponet_hand == 'A') {
				score += get_score('C');
			} else if (opponet_hand == 'B') {
				score += get_score('A');
			} else if (opponet_hand == 'C') {
				score += get_score('B');
			}
		} else {
			// Win
			if (opponet_hand == 'A') {
				score += get_score('B');
			} else if (opponet_hand == 'B') {
				score += get_score('C');
			} else if (opponet_hand == 'C') {
				score += get_score('A');
			}
		}

		return score;
	}

public:
	rps_player();

	// Play a round
	uint16_t play_round(const std::string &line) const;
};

#endif
