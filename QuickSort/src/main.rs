use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let mut unorded: Vec<u32> = (0..20).collect();
    unorded.shuffle(&mut thread_rng());
    println!("{:?}", unorded);
    let ordered: Vec<u32> = quick_sort(unorded);  
    println!("{:?}", ordered);
}

fn quick_sort(input: Vec<u32>) -> Vec<u32> {
    if input.len() <= 1 {
        return input;
    }
    let pivot = input[0];
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for i in 1..input.len() {
        if input[i] < pivot {
            left.push(input[i]);
        } else {
            right.push(input[i]);
        }
    }

    let mut result: Vec<u32> = Vec::new();
    result.append(&mut quick_sort(left));
    result.push(pivot);
    result.append(&mut quick_sort(right));
    result


}