fn main() {
    // This is a one line comment
    /*
    This is a block comment
    */

    // basic print
    println!("Hello, world!");

    // formatted print
    let x= 5;
    println!("Variable = {}", x);
    println!("Variable = {x}");

    // formatted print with inner variables
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{number:>5}", number=1);
    println!("{number:0>5}", number=1);
    println!("{number:0<5}", number=1);

    // decimal limit formatted print
    let pi = 3.141592;
    println!("Pi is roughly {pi:.3}");

    // debug formatted print
    println!("{:?}", x);
}
