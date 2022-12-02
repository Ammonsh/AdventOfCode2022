
struct StrategyGuide {
    // choices is a vector of tuples
    choices: Vec<(char, char)>,
}

impl StrategyGuide {
    // Create a new StrategyGuide from a string of choices
    fn from_string(string: &str) -> StrategyGuide {
        let mut choices = Vec::new();

        // Split the string into lines, then split each line into opponent's choice and player's choice
        for line in string.lines() {
            let choice: Vec<char> = line.trim().chars().collect();
            choices.push((choice[0], choice[2]));
        }

        StrategyGuide {
            choices,
        }
    }

    // Calculate the total score for this strategy guide
    fn calculate_score1(&self) -> i32 {
        let mut total_score = 0;

        // Iterate over all choices in the map and apply the scoring rules
        for (opponent_move, my_move) in self.choices.clone() {
            // If the player chooses the same shape as the opponent, it's a draw
            if opponent_move == 'A' && my_move == 'X' {
                // Rock vs Rock
                total_score += 1 + 3; // Rock score + draw score
            } else if opponent_move == 'A' && my_move == 'Y' {
                // Rock vs Paper
                total_score += 2 + 6; // Paper score + win score
            } else if opponent_move == 'A' && my_move == 'Z' {
                // Rock vs Scissors
                total_score += 3 + 0; // Scissors score + lose score
            } else if opponent_move == 'B' && my_move == 'X' {
                // Paper vs Rock
                total_score += 1 + 0; // Rock score + lose score
            } else if opponent_move == 'B' && my_move == 'Y' {
                // Paper vs Paper
                total_score += 2 + 3; // Paper score + draw score
            } else if opponent_move == 'B' && my_move == 'Z' {
                // Paper vs Scissors
                total_score += 3 + 6; // Scissors score + win score
            } else if opponent_move == 'C' && my_move == 'X' {
                // Scissors vs Rock
                total_score += 1 + 6; // Rock score + win score
            } else if opponent_move == 'C' && my_move == 'Y' {
                // Scissors vs Paper
                total_score += 2 + 0; // Paper score + lose score
            } else if opponent_move == 'C' && my_move == 'Z' {
                // Scissors vs Scissors
                total_score += 3 + 3; // Scissors score + draw score
            }
        }

        total_score
    }
    fn calculate_score2(&self) -> i32 {
        let mut total_score = 0;

        // Iterate over all choices in the map and apply the scoring rules
        for (opponent_move, my_move) in self.choices.clone() {
            // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win.
            if my_move == 'X' {
                if opponent_move == 'A' {
                    // Need to lose so we choose Scissors
                    total_score += 3 + 0; // Scissors score + lose score
                } else if opponent_move == 'B' {
                    // Need to lose so we choose Rock
                    total_score += 1 + 0; // Rock score + lose score
                } else if opponent_move == 'C' {
                    // Need to lose so we choose Paper
                    total_score += 2 + 0; // Paper score + lose score
                }
            } else if my_move == 'Y' {
                // Y means you need to end the round in a draw
                if opponent_move == 'A' {
                    // Need to draw so we choose Rock
                    total_score += 1 + 3; // Rock score + draw score
                } else if opponent_move == 'B' {
                    // Need to draw so we choose Paper
                    total_score += 2 + 3; // Paper score + draw score
                } else if opponent_move == 'C' {
                    // Need to draw so we choose Scissors
                    total_score += 3 + 3; // Scissors score + draw score
                }
            } else if my_move == 'Z' {
                // Z means you need to win
                if opponent_move == 'A' {
                    // Need to win so we choose Paper
                    total_score += 2 + 6; // Paper score + win score
                } else if opponent_move == 'B' {
                    // Need to win so we choose Scissors
                    total_score += 3 + 6; // Scissors score + win score
                } else if opponent_move == 'C' {
                    // Need to win so we choose Rock
                    total_score += 1 + 6; // Rock score + win score
                }
            }
        }

        total_score
    }
}

fn main() {
    let strategy_guide_input = std::fs::read_to_string("puzzle_input.txt").unwrap();
    // let strategy_guide_input = "A Y
    //     B X
    //     C Z";

    let guide = StrategyGuide::from_string(&strategy_guide_input);

    let total_score1 = guide.calculate_score1();
    println!("Total score: {}", total_score1);

    let total_score2 = guide.calculate_score2();
    println!("Total score: {}", total_score2);
}
