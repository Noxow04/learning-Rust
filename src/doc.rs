fn main() {
    // This is a one line comment
    /*
    This is a block comment
    */

    /* print */
    // basic print
    println!("Hello, world!");

    // formatted print
    let x = 5;
    println!("Variable = {}", x);
    println!("Variable = {x}");

    // formatted print with inner variables
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{number:>5}", number = 1);
    println!("{number:0>5}", number = 1);
    println!("{number:0<5}", number = 1);

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
    /* Integer operations
    int + int   for ex 1i32 + 2 = 3i32
    int - int   for ex 1i32 - 2 = -1i32     but 1u32 - 2 is an Error */

    /* Boolean logic
    bool AND bool <=> bool && bool
    bool OR bool <=> bool || bool
    NOT bool <=> !bool */

    /* Use underscores to improve readability!
    1 million is 1_000_000 */


    /* Tuples */
    // Tuples are immutable arrays and can have items with different types
    let tuple = (true, 10, 0.5);

    // Tuples can be destructured to bind each of their item to an independent variable
    let (x1, x2, x3) = tuple;
    let result = x2 + x3;

    // Tuples with more than 12 items wont print
    // Tuples can be nested
    let tuple_of_tuples = ((1, 2, 3), (4, 5), (6, 7, 8));


    /* Arrays, slicing and indexing */
    // Arrays have a fixed length and type
    // syntax: [type; length]
    let array: [i32; 5] = [1,2,3,4,5];

    // Arrays can also be initialized with a default value
    // Syntax : let array = [value; length]
    let array = [5; 3];
    println!("[5; 3] is equivalent to [5,5,5] : {}", array == [5,5,5])

    /* Indexing
    array[0] <=> first item of the array
    array[1] <=> second item of the array
    etc...
    Indexing outside the bounds of the array causes an error
     */

    /* Length
    array.len() <=> length of the array
     */

    /* Slicing
    &array <=> whole array as a slice
    &array[0..5] <=> slice of the 5 (0,1,2,3,4) first items of the array
     */
}
