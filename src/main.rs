#![allow(unused)]         // hide warnings for unused code

mod doc;    // importing the doc.rs module

fn main() {
    doc::main();    // using the main function of the doc module
    println!("{:?}", fibonacci(10))
}

fn fibonacci(n: usize) -> Vec<u32> {
    /// function that returns a vector of the n first numbers of the fibonacci sequence (starting at 0)
    let mut list = vec![0_u32];         // using a vector because arrays have a fixed length
    let mut previous_value: u32 = 0;
    let mut current_value: u32 = 1;
    for index in 0..n {
        current_value += previous_value;
        previous_value = list[index];
        list.push(current_value);
    }
    list
}
