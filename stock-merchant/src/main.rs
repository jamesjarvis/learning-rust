use std::io;
use std::collections::HashMap;

fn main() {

    // to get the bullshit
    let mut size = String::new();

    io::stdin().read_line(&mut size)
        .expect("Failed to read line");

    let size: usize = match size.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let mut array = String::new();
    
    io::stdin().read_line(&mut array)
        .expect("Failed to read line");

    let mut colourcount = HashMap::new();

    for w in array.split_whitespace() {
        let x: i32 = w.parse().expect("B");

        // Get from map
        match colourcount.get(&x) {
            // If it exists, increment the count
            Some(&count) => colourcount.insert(x, count+1),
            // If it doesnt already exist, add to map
            _ => colourcount.insert(x, 1),
        };
    }


    let mut count_socks: i32 = 0;
    for (v, i) in colourcount {
        count_socks = count_socks + (i /2);
        // println!("{}, count {}", v, i);
    }

    println!("{}",count_socks);

}