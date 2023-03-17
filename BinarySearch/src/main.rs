

fn main() {
    let mut numbers: Vec<i32> = (1..101).collect();
    let message: String = binary_search(&mut numbers, 43);
    println!("{}", message);

}


fn binary_search(numbers: &mut Vec<i32>, target: i32) -> String {
    println!("target is: {:?}", target);
    while numbers.len() > 1 {
        let index: usize = &numbers.len() / 2;
        println!("Index: {}", index);
        if target == numbers[index] {
            return format!("We found it at index: {}", index);
        }
        else if target < numbers[index] {
            numbers.drain(index..numbers.len());
            println!("New numbers: {:?}", numbers);
        }
        else {
            numbers.drain(0..index);
            println!("New numbers: {:?}", numbers);
        }
    }
    return format!("We didn't find it");
}   