use std::io;
use std::io::BufRead;

type Commit = String;

// A Log represents a line in the input: a commit and its parents.
struct Log {
    commit: Commit,
    parents: Vec<Commit>,
}

use std::collections::HashMap;
use std::collections::HashSet;

// Enter your code here. Please read the code above and below before starting.
fn count_ancestors(root: Commit, logs: Vec<Log>) -> i32 {
    let mut allParents = HashMap::new();
    for log in logs {
        allParents.insert(log.commit, log.parents);
    }

    let mut visited = HashSet::new();
    let mut queue = Vec::new();

    visited.insert(root.to_string());
    queue.push(root.to_string());

    let mut top = String::new();

    while queue.len() != 0 {
        match queue.pop() {
            Some(temptop) => top = temptop.to_string(),
            _ => {}
        }
        let mut parents = String::new();
        // let top = queue.pop();
        match allParents.get(&top) {
            // If the commit exists
            Some(parents) => {
                // Go through parents
                for parent in parents {
                    // If it has not been visited yet, add it to visited, and add it to the queue
                    if !visited.contains(parent) {
                        visited.insert(parent.to_string());
                        queue.push(parent.to_string());
                    }
                }
            },
            _ => println!("oops, cannot find {}", top),
        }
    }

    let mut length = 0;
    for _ in visited {
        length += 1;
    }
    return length;
}


fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let root = lines.next().unwrap().unwrap().trim().to_string();
    let mut logs: Vec<Log> = vec![];
    for l in lines {
        let l = l.unwrap();
        let mut cs = l.split_whitespace();
        let commit = String::from(cs.next().unwrap());
        let parents = cs.map(String::from).collect();
        logs.push(Log { commit, parents });
    }
    println!("{}", count_ancestors(root, logs));
}