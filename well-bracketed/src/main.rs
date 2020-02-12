use std::io;
use std::io::Read;


// Idea being that this checks whether a string of brackets is "well balanced":
// () = true
// (() = false
// ((((aklsjcb)dsfv)s)) = true

// Enter your code here.
fn is_well_balanced(s: &str) -> bool {
    let mut vec = Vec::new();

    for c in s.chars() {
        // If character is opening parenthese, add to stack
        if c == '(' { 
            vec.push(c);
        } else if c == ')'{
            // If the stack is empty, just fail
            if vec.len() == 0 {
                return false;
            }
            // If it is the closing parenthese, pop from stack and compare
            let other = vec.pop();
            if other != Some('(') {
                return false;
            }
        }
    }

    // If length is 0 return true, else false
    if vec.len() == 0 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", is_well_balanced(&s));
}