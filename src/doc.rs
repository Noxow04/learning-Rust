#![allow(dead_code)]    // hide warnings for unused code
#![allow(unused_qualifications)]

/* Structures */
// Unit struct
struct Unit;

// Tuple struct
struct Pair(u32, u32);

// C like struct
struct Point {
    x: f32,
    y: f32,
}

// Structures can be used as fields for another structure (they must be declared prior however)
struct Box {
    top_left_corner: Point,
    width: u32,
    height: u32,
}


/* Enums */
// Basic example : using an enum to manage events on a web page
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
} // note that inner classes arent declared prior


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
    let variable = true;


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
    let tuple = (true, 10, 5, 0.5);

    // Tuples can be destructured to bind each of their item to an independent variable
    let (x1, x2, x3, x4) = tuple;
    let result = x2 + x3;

    // Tuples with more than 12 items wont print
    // Tuples can be nested
    let tuple_of_tuples = ((1, 2, 3), (4, 5), (6, 7, 8));


    /* Arrays, slicing and indexing */
    // Arrays have a fixed length and type
    // syntax: [type; length]
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    // Arrays can also be initialized with a default value
    // Syntax : let array = [value; length]
    let array = [5; 3];
    println!("[5; 3] is equivalent to [5,5,5] : {}", array == [5, 5, 5]);

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


    /* Structures (declared before the main function)
    Structures are 'personal' types
    There are basically three types of structures :
        - Tuple structs, which are named tuples
        - The classic C structs
        - Unit structs, which are field-less
    */
    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Make a new point by using struct update syntax to use the fields of an other one
    let bottom_right = Point { x: 5.2, ..point };

    // Access the fields of a struct
    let left_edge = point.x;
    let top_edge = point.y;
    let Point { x: left_edge, y: top_edge } = point;    // Equivalent

    // Instantiate a structure
    let unit = Unit;
    let pair = Pair(2, 5);

    // Access the fields of a tuple struct
    let integer_1 = pair.0;
    let integer_2 = pair.1;
    let Pair(integer_1, integer_2) = pair; // Equivalent


    /* Enums (declared before the main function)
    Enums are a collection of classes
    Though only one of the inner classes can be assign to a variable at a time
    */
    // assignment with enums
    let load = WebEvent::PageLoad;
    let click = WebEvent::Click { x: 50, y: 50 };

    // use statement to avoid manual scoping
    use WebEvent::{PageLoad, KeyPress};
    use WebEvent::*; // apply use statement for each name inside WebEvent

    // assignment without scoping
    let load = PageLoad;
    let click = Click { x: 50, y: 50 };
}
