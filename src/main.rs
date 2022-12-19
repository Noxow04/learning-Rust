fn main() {
    println!("{:?}", fibonacci(10))
}

fn fibonacci(n: usize) -> Vec<u32> {
    let mut list = vec![0u32];
    let mut previous_value: u32 = 0;
    let mut current_value: u32 = 1;
    for index in 0..n {
        current_value += previous_value;
        previous_value = list[index];
        list.push(current_value);
    }
    list
}