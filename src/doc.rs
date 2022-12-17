fn main() {
    // This is a one line comment
    /*
    This is a block comment
    */

    /* print */
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
    println!("{:#?}", x);


    /* variable types
    signed integers : i8 / i16 / i32 (default) / i64 / i128
    unsigned integers : u8 / u16 / u32 / u64 / u128
    floats : f32 / f64 (default)
    unicode characters : char
        such as 'a', should be single-quoted
    boolean : bool
    */
    // Variables can be type annotated
    let logical: bool = true;   // not useful since true can only be a type 'bool'
    let a_float: f64 = 1.0;     // regular annotation
    let an_int = 5i32;      // suffix annotation (less readable in my opinion)

    // Or a default type is assigned
    let default_logical = true;
    let default_float = 1.0;
    let default_int = 5;

    // Have to specify if a variable can be changed
    let mut variable = 10;
    variable = 15;
    /* This doesnt work
    let variable = 10;
    variable =15;
    */

    // A variable cant change type
    /* This doesnt work
    variable = true;
    */

    // But it can be overwritten
    let mut variable = true;


    /* Operators */
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}
