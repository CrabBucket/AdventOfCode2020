use std::{fs, path::Path, env};
pub fn findpairthenmult(input: u32) -> u32{
    input * (2020u32 - input)
}
pub fn day1(){
    // let path = env::current_dir().expect("error getting current dir").join("day1.txt");
    // println!("{:?}", path);
    let inputs = fs::read_to_string("day1.txt").expect("couldn't find file");
    let mut numbers = Vec::new();
    for pair in inputs.split("\n"){
        let temp = &pair.replace("\r", "").to_string();
        // println!("{:?}",temp);
        let first = temp.parse::<u32>().unwrap();
        numbers.push(first);
        
        
    }
    for first in numbers.clone() {
        for second in numbers.clone() {
            if first + second == 2020u32{
                println!("{}",findpairthenmult(first));
            }
        }
    }
}