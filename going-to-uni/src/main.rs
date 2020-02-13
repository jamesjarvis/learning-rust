use std::io;
use std::io::Read;

fn notnice(x: u64, y: u64) -> u64 {
    let mut array = vec![vec![0u64; (y+1) as usize]; (x+1) as usize];

    // println!("x, y = {}, {}", x, y);

    array[0][0] = 1;
    for i in 0usize..=(x as usize) {
        for j in 0usize..=(y as usize) {
            if i > 0 {
                array[i][j]+= array[i-1][j];
            }
            if j > 0 {
                array[i][j]+= array[i][j-1];
            }
            array[i][j] = array[i][j] % 1000;
            // println!("{},{} = {}",i, i, array[i][j]);
        }
    }
    // println!("array shit = {}", array[(x) as usize][(y) as usize]);
    return array[(x) as usize][(y) as usize];
}
 



// Enter your code here.
fn count_routes(mut x: i32, mut y: i32) -> i32 {

    if x == 0 || y == 0 {
        return 1;
    }


    let xn = x.abs() as u64;

    let yn = y.abs() as u64;

    let facboi = notnice(xn, yn);

    return (facboi % 1000) as i32;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let x: i32 = iter.next().unwrap().parse().unwrap();
    let y: i32 = iter.next().unwrap().parse().unwrap();
    println!("{}", count_routes(x, y));
}