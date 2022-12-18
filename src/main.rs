fn main() {
    println!("{:?}", fibonacci(10))
}

fn fibonacci(n: usize) -> Vec<u64> {
    let mut list = vec![0];
    let mut previous_value: u64 = 0;
    let mut value: u64 = 1;
    for iter in 0..n {
        value += previous_value;
        previous_value = list[iter];
        list.push(value);
    }
    list
}