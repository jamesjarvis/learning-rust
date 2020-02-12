use std::io;
use std::io::Read;

// Enter your code here.
fn max_len(lengths: Vec<i32>) -> i32 {
    // First of all, idk, get the min of the list?
    let mut min = 1000000;
    for v in lengths {
        if (v < min) {
            min = v;
        }
    }

    // Sort it and remove all elements apart from the top 100
    // Sort that shiz
    lengths.sort();

    let temp = lengths;
    // Remove all but top 100
    for v in lengths {
        
    }


}


fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lengths: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{}", max_len(lengths));
}