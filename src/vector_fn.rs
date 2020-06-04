fn main() -> () {
    let a = vec![1,2,3];
    let b = vec![2,3,4];
    let c = diff(a.clone(), b.clone());
    let d = overlap(a.clone(), b.clone());
    println!("{:?}", c);
    println!("{:?}", d);
}

// Calculate the difference between two input vectors.
pub fn diff(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    for x in a.iter() {
        let mut present: bool = false;
        for y in b.iter() {
            if *x == *y {
                present = true;
            }
        }
        if !present {
            output.push(*x);
        }
    }

    return output;
}

// Calculate the overlap between two input vectors.
pub fn overlap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    for x in a.iter() {
        let mut present: bool = false;
        for y in b.iter() {
            if *x == *y {
                present = true;
            }
        }
        if present {
            output.push(*x);
        }
    }

    return output;
}