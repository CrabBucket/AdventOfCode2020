use Day1::day1;

mod Day1;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("error reading line");

    let value: u32 = input.trim().parse().unwrap();
    match value {
        1 => {
            day1();
        }
        _ => {
            println!("Enter A number 1-31");
        }
    };
}
