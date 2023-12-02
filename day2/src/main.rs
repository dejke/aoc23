use std::fs;

fn main() {
    let file =  fs::read_to_string("./input").unwrap();
    let lines = file.lines();

    let max_r = 12;
    let max_g = 13;
    let max_b = 14;

    let mut tally = 0;
    'game:
    for line in lines{
        
        let mut split_line = line.trim_start_matches("Game ").split(":");
        let game_id = split_line.next().unwrap().parse::<i32>().unwrap();
        
        for subsets in split_line.next().unwrap().split(";"){ 
            
            let  (mut red, mut green, mut blue) = (0,0,0);
            
            for cubes in subsets.split(','){
                print!("{}\n", cubes);
                let mut cubes_iter = cubes.split_whitespace();
                let count = cubes_iter.next().unwrap().parse::<i32>().unwrap();
                let color = cubes_iter.next().unwrap();

                match color{
                    "red" => red += count,
                    "green" => green += count,
                    "blue" => blue += count,
                    _ => print!("invalid format")
                }
            }

            if red > max_r || green > max_g || blue > max_b{
                continue 'game;
            }
        }
        tally += game_id;
    }   
    print!{"{}", tally};
}


