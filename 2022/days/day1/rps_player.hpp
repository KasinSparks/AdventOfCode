#ifndef AOC_RPS_PLAYER_HPP
#define AOC_RPS_PLAYER_HPP

#include <cstdint>
#include <exception>
#include <string>

class rps_player {
private:
	uint16_t clacluate_score(const char &opponet_hand, const char &players_hand) const {
		uint16_t score = 0;
		char adj_players_hand;
		
		// Get the value from the hand the player showed
		switch (players_hand) {
			case 'X': { score += 1; adj_players_hand = 'A'; break; }
			case 'Y': { score += 2; adj_players_hand = 'B'; break; }
			case 'Z': { score += 3; adj_players_hand = 'C'; break; }
			default: throw new std::exception();
		}

		// Get the value from win or loss
		if (adj_players_hand == opponet_hand) {
			// Tie
			score += 3;
		} else if (opponet_hand == 'A' && adj_players_hand == 'B') {
			// Win
			score += 6;
		} else if (opponet_hand == 'B' && adj_players_hand == 'C') {
			// Win
			score += 6;
		} else if (opponet_hand == 'C' && adj_players_hand == 'A') {
			// Win
			score += 6;
		} else {
			// Loss
			score += 0;
		}

		return score;
	}

public:
	rps_player();

	// Play a round
	uint16_t play_round(const std::string &line) const;
};

#endif
