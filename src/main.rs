use std::fs;

fn main() {
    // --snip--
    let file_path = "input.dat";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let line = contents.split('\n');
    let mut solution = 0;
    for part in line {
        let mut first_num = 0;
        let mut last_num = 0;
        let mut numbers = Vec::new();
        for char in part.chars() {
            if char.is_digit(10) {
                // println!("There is a {} in the line", char);
                numbers.push(char);

            }
        }
        if numbers.len() == 0 {
            break;
        }
        first_num = numbers[0] as i32;
        last_num = numbers[numbers.len()-1] as i32;
        println!("{}", first_num);
        println!("{}", last_num);
        solution = solution + (first_num * 10) + last_num;
        println!("Onto the next line");

    }
    println!("{}", solution);
}
