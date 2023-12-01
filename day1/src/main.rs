use std::fs;

fn main(){
    let file =  fs::read_to_string("./input").unwrap();
    let lines =  file.lines();
   
    let sum = lines.map(code_from_line).reduce(|a,b| a+b).expect("Could not find the codes");
    print!("{}", sum);
}

fn code_from_line(line: &str) -> u32{
    let chars:Vec<char> = line.chars().collect();

    let first_index = line.find(|c:char| c.is_digit(10)).expect("Could not find digits in line");
    let last_index = line.rfind(|c:char| c.is_digit(10)).expect("Could not find digits in line");

    return chars[first_index].to_digit(10).unwrap() * 10 + chars[last_index].to_digit(10).unwrap();
}