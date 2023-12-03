use std::fs;


fn main() {
    let lines: Vec<Vec<char>> =  fs::read_to_string("./input").unwrap().lines().map(|l| l.chars().collect()).collect();
    
    let mut sum = 0;

    for (row, line) in lines.iter().enumerate(){
        let mut num_start = usize::MAX;
        
        for (col, c) in line.iter().enumerate(){
            if num_start == usize::MAX && c.is_digit(10){
                num_start = col;
            }
            if num_start != usize::MAX && !c.is_digit(10){
                let num_end = col;
                sum += part_value(row, num_start, num_end, &lines);
                num_start = usize::MAX;
            }
            
        }

        if num_start != usize::MAX{
            //Edge case at the end of the line
            let num_end = line.len();            
            sum += part_value(row, num_start, num_end, &lines);
        }
    }

    print!("{}", sum);
}

fn part_value(part_row:usize, num_start:usize, num_end:usize, lines: &Vec<Vec<char>>) -> u32{
    let mut found_symbol = false;
    let line_length = lines[0].len();
    let rows_start = if part_row == 0 {0} else {part_row-1};
    let rows_end = if part_row >= lines.len() - 1 {lines.len()} else {part_row + 2};

    for row in rows_start..rows_end{
        let col_start = if num_start == 0 {0} else {num_start-1};
        let col_end = if num_end == line_length {line_length} else {num_end+1};

        for col in col_start..col_end{
            let c = lines[row][col];
            if !c.is_digit(10) && c != '.'{
                found_symbol = true;
                break;
            }
        }
    }

    if !found_symbol{
        return 0;
    }

    let mut value = 0;
    for col in num_start..num_end{
        value = value * 10 + lines[part_row][col].to_digit(10).unwrap();
    }
    return value;
}