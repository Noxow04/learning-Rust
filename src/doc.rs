#![allow(unused)]         // hide warnings for unused code

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


/* Constants and statics
Global variables are defined outside the main function
Their type must be specified
const are variables that cant be changed
static can be 'mut'
However both are defined at compile time
By convention global variables are written in MARCO_CASE
 */
static LEN: usize = 5;
static mut THRESHOLD: u32 = 10;
const LETTER: char = 'a';


/* From statement */
#[derive(Debug)]        // implement debug methods (such as debug print) for the struct underneath
struct Number {
    value: i32,
}

// implementing the from statement for the 'Number' struct
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

pub(crate) fn main() {  // The 'pub(crate)' expression make the function public for usage in the main file
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
    let pi = std::f64::consts::PI;
    println!("Pi is roughly {pi:.3}");

    // debug formatted print
    println!("{:?}", x);


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
    let an_int = 5i32;      // suffix annotation can be written 5_i32 for readability

    // Or a default type is assigned
    let default_logical = true;
    let default_float = 1.0;
    let default_int = 5;

    // Have to specify if a variable can be changed
    let mut variable = 10;
    variable = 15;
    /* Error !
    let variable = 10;
    variable =15;
     */

    // A variable cant change type
    /* Error !
    variable = true;
     */

    // But it can be overwritten (doesnt have to change type)
    let variable = true;

    // variable can interfere with other
    let mut vec = vec![];   // this vector has the type Vec<_> because no inner type is specified
    vec.push(1_u8);            // now it has the type Vec<u8>  because it contain an u8 type variable


    /* Operators */
    /* Integer operations
    int + int   for ex 1i32 + 2 = 3i32
    int - int   for ex 1i32 - 2 = -1i32     but 1u32 - 2 is an Error
     */

    /* Boolean logic
    bool AND bool <=> bool && bool
    bool OR bool <=> bool || bool
    NOT bool <=> !bool
     */

    /* Use underscores to improve readability!
    1 million is 1_000_000
     */


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


    /* Vectors
    Vectors are array-like structures but can change length
    However, because they are a structure and not a proper type,
    Vectors are declared with a macro (or the new() method)
     */
    // an empty vector
    let mut vec: Vec<i32> = vec![];         // note that the vector type must somehow be specified to avoid errors
    let mut vec: Vec<i32> = Vec::new();

    // a vector with values
    let mut vec = vec![1, 2, 3];

    // putting a new value inside a vector
    vec.push(4);

    // vectors are treated like arrays for indexing (and slicing)
    let three = vec[2];
    println!("{}", three);


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
    let top_left = Point { ..point };

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


    /* Scope and shadowing
    Each variable exists and shadows previous ones in their own block (and inner ones)
     */
    // This variable have the biggest scope because it is declared directly inside the main function
    let a_variable = 10;
    let mut another_variable = 10;

    // This is an indented block (it has no other influence than scope)
    {
        // This variable will shadow the first one inside this block
        let a_variable = 42;
        // However changing the value of a mutable variable will be effective outside of the block scope
        another_variable = 42;

        /* Here
        a_variable == 42
        another_variable = 42
         */
    }

    /* Here
    a_variable == 10
    another_variable = 42
     */
    // note that shadowing also work outside of blocks (see above in 'Variable types')


    /* Declaring first */
    // Variable can be declared and assigned a value later
    let variable;
    variable = 10;

    // However declared variables without being initialized cant be used in other expression
    /* Error !
    let variable: i32;          note that a type can be specified in the declaration
    println!("{}", variable);
     */


    /* Freezing
    When a mutable variable is shadowed by an immutable one,
    its value cant be modified inside the scope of the shadowing variable
     */
    let mut variable = 20;
    {
        let variable = variable;
        /* Error !
        variable = 20;
         */
    }
    // The variable is unfreeze outside the block
    variable = 20;


    /* Casting */
    // Implicit conversions dont work
    let float = 6.3;
    /* Error !
    let integer: u8 = float;
     */

    // Explicit conversion (using casting)
    let integer = float as u8;

    /* Casting is limited, for example you cant cast a float as a char
    It has to be cast as an integer first :
    let character = float as u8 as char;        note that you can have multiple castings on a single expression
     */

    /* WARNING
    When casting, be careful with the types because unwanted results can occur
    when castings values into types too small (bitwise)
    for example :
    1000_u16 as u8 == 232_u8
    232_u8 as i8 == -24_i8
     */


    /* From and Into traits */
    // The from trait provide a conversion between types
    let str = "hello";
    let string = String::from(str);     // &str -> String

    // it can be implemented for your own type (see before the main function)
    let num = Number::from(30);
    let num = Number { value: 30 };                 // equivalent in this example

    // The into trait is the reciprocate of the from trait
    // (and call the from trait, so there is no need to implement it)
    let int = 5;
    let num: Number = int.into();   // The type of the variable must be specified
    let num = Number { value: int };
}

fn answer() -> i32 { 42 }

#[test]     // 'use cargo test' to run these
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}
