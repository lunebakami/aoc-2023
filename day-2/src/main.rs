fn main() {
    let games = std::fs::read_to_string("./input")
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();

    let mut result = 0;

    games.iter().for_each(|game| {
        // Remove Game X: from the string
        let game_label = game.split(":").nth(0).unwrap().trim();
        let game_id = game_label.split(" ").nth(1).unwrap().parse::<i32>().unwrap();

        let game = game.split(": ").nth(1).unwrap();

        // Split games into rounds
        let rounds = game.split(";").collect::<Vec<_>>();

        let has_invalid_round = rounds.iter().find(|round| {
            // Split round into matches
            let matches = round.split(",").collect::<Vec<_>>();

            let has_invalid_match = matches.iter().find(|m| {
                let m = m.trim().split(" ").collect::<Vec<_>>();

                let (qty, color) = (m[0], m[1]);

                let mut invalid_match = false;
                match color {
                    "red" => {
                        if qty.parse::<i32>().unwrap() > 12 {
                            invalid_match = true;
                        }
                    }
                    "green" => {
                        if qty.parse::<i32>().unwrap() > 13 {
                            invalid_match = true;
                        }
                    }
                    "blue" => {
                        if qty.parse::<i32>().unwrap() > 14 {
                            invalid_match = true;
                        }
                    }
                    _ => println!("unknown"),
                };

                invalid_match
            });

            match has_invalid_match {
                Some(_) => true,
                None => false,
            }
        });

        match has_invalid_round {
            Some(_) => (),
            None => {
                result = result + game_id;
            }
        };
    });

    println!("Result: {}", result);
}
