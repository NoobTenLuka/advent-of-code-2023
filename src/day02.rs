pub fn part1(input: String) -> String {
    input
        .lines()
        .filter_map(|line| {
            let (game, game_info) = line.split_once(':').unwrap();
            let game_id = game[5..].parse::<i32>().unwrap();

            let mod_game_info = game_info.replace(';', ",");

            if mod_game_info.split(',').any(|draw| {
                let (num_str, color) = draw.trim().split_once(' ').unwrap();
                let num = num_str.parse::<i32>().unwrap();

                if num > 14 {
                    return true;
                }

                if num == 14 && color != "blue" {
                    return true;
                }

                if num == 13 && color == "red" {
                    return true;
                }

                false
            }) {
                return None;
            }

            Some(game_id)
        })
        .sum::<i32>()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let (_, game_info) = line.split_once(':').unwrap();
            let mod_game_info = game_info.replace(';', ",");

            let colors = mod_game_info.split(',').fold([0, 0, 0], |mut acc, draw| {
                let (num_str, color) = draw.trim().split_once(' ').unwrap();
                let num = num_str.parse::<i32>().unwrap();
                let color_index = match color {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => unreachable!(),
                };

                if acc[color_index] < num {
                    acc[color_index] = num
                }

                acc
            });

            colors.iter().product::<i32>()
        })
        .sum::<i32>()
        .to_string()
}
