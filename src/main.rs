use std::fs;

fn main() {
    // --snip--
    let file_path = "input.dat";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut data;
    for line in contents {
        for char in line {
            if char.is_digit(10) {
                println!("There is a {} in the line", char);
            }
        }
    }
 //   println!("With text:\n{contents}");
}
