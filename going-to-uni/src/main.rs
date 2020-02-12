use std::io;
use std::io::Read;
// use std::cmp;

// fn plusplus(x: i32) -> i32{
//     if x == 0 {
//         return 0;
//     }
//     return x + plusplus(x-1);
// }

// fn fac(n: f64) -> f64 {
//     if n == 0.0 {
//         return 0.0;
//     }
//     if n == 1.0 {
//         return 1.0;
//     }
//     return n * fac(n-1.0);
// }

fn count_routes_rec(x: i32, y: i32) -> i32 {
    if x == 0 || y == 0 {
        return 1;
    }
    return count_routes_rec(x-1, y) + count_routes_rec(x, y-1);
}

// Enter your code here.
fn count_routes(mut x: i32, mut y: i32) -> i32 {
    // Fix issues with negatives
    // if x < 0 && y < 0 {
    //     x = x.abs();
    //     y = y.abs();
    // } else if x < 0 && x ==  {

    // }


    x = x.abs();
    // let xn = f64::from(x.abs());
    y = y.abs();
    // let yn = f64::from(y.abs());

    let count = count_routes_rec(x, y);

    return count % 1000;
    // if x == 0 || y == 0 {
    //     return 1;
    // }

    // println!("x: {}, y:{}", x, y);

    // let facx = fac(xn);
    // let facy = fac(yn);
    // let facxy = fac(xn + yn);

    // println!("{}, {}, {}", facx, facy, facxy);

    // // return facxy / (facx * facy);
    // return 0;

    // Alright alright, this seems to be binomial coefficient
    
    // if x == 0 || y == 0 {
    //     return 1;
    // }
    // return count_routes(x-1, y) + count_routes(x, y-1); 



    // let mut count = 0;
    // // Simple paths (with only 1 change) are x + y
    // let simple_paths = x + y;
    // // count += simple_paths;
    // // count - 1;

    // for _xi in 1..x.abs()+1 {
    //     for _yi in 1..y.abs()+1 {
    //         count += 1;
    //     }
    // }

    // return count;

    // 
}
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let x: i32 = iter.next().unwrap().parse().unwrap();
    let y: i32 = iter.next().unwrap().parse().unwrap();
    println!("{}", count_routes(x, y));
}