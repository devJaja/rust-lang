
// compound data types
//     -&str and String
//     -Arrays
//     -Vectors
//     -Tuples
//     -Empty Tuples


fn main() {
    // Variables and Mutability
    // variable is a name that represents a value stored in memory
    let x: i32 = 10; // immutable variable
    println!("The value of x is: {}", x);
    let mut y: i32 = 20; // mutable variable
    println!("The initial value of y is: {}", y);
    y = 30; // modifying the mutable variable
    println!("The modified value of y is: {y}");


    // Shadowing
    // shadowing allows you to declare a new variable with the same name as a previous variable
    // the new variable shadows the previous one
    // this is useful for transforming data while keeping the same variable name
    let z: i32 = 5; // initial declaration
    println!("The initial value of z is: {}", z);
    let z: i32 = z + 10; // shadowing the previous z
    println!("The shadowed value of z is: {}", z);
    let z: i32 = z * 2; // shadowing again
    println!("The shadowed value of z after multiplication is: {}", z);

    // constants
    // constants are values that cannot be changed and must have a type annotation
    const MAX_POINTS: u32 = 100_000; // constant declaration
    println!("The maximum points are: {}", MAX_POINTS);

    // primitive data types review
    // primitive data types are the most basic data types in Rust
    // - integers
    // - floating-point numbers
    // - booleans
    // - characters

    // integer types
    let a: u8 = 255; // unsigned 8-bit integer
    let b: i8 = -128; // signed 8-bit integer
    let c: u16 = 65_535; // unsigned 16-bit integer
    let d: i16 = -32_768; // signed 16-bit integer
    let e: u32 = 4_294_967_295; // unsigned 32-bit integer
    let f: i32 = -2_147_483_648; // signed 32-bit integer
    let g: u64 = 18_446_744_073_709_551_615; // unsigned 64-bit integer
    let h: i64 = -9_223_372_036_854_775_808; // signed 64-bit integer

    println!("Integer values: {}, {}, {}, {}, {}, {}, {}, {}", a, b, c, d, e, f, g, h);

    // floating-point types
    let float_32: f32 = 3.14; // 32-bit floating point
    let float_64: f64 = 2.718281828459045; // 64-bit floating point
    println!("Floating-point values: {}, {}", float_32, float_64);

    // boolean type
    let is_rust_fun: bool = true; // boolean value
    let is_math_hard: bool = false; // boolean value
    println!("Is Rust fun? {}, Is math hard? {}", is_rust_fun, is_math_hard);

    // character type
    let letter_a: char = 'A'; // character type
    let emoji_smile: char = '😊'; // character type

    // type alliases
    // you can create type aliases using the 'type' keyword
    type Kilometers = i32; // creating a type alias for i32
    let distance: Kilometers = 100; // using the type alias
    println!("Distance in kilometers: {}", distance);

    // type conversion
    let int_value: i32 = 42;
    let float_value: f64 = int_value as f64; // converting i32 to f64
    println!("Integer value: {}, Converted float value: {}", int_value, float_value);

    //primitive data types review
    let num : i32 = 5;
    println!("Here is the number : {num}");
    // string
    let fixed_str: &'static str = "String with fixed length"; // immutable, fixed size
    let mut growable_string: String = String::from("Growable String"); // mutable, dynamic size
    growable_string.push_str("- add to more content"); // add characters to the end
    println!("Here is a fixed string: {}", fixed_str);
    println!("this is the growable array : {}", growable_string);

    // arrays
    // arrays have a fixed size and can hold elements of the same type
    let fixed_size_array: [i32; 3] = [1, 2, 3]; // fixed size array
    let mut mutable_array: [i32;3] = [10, 20, 30]; // mutable fixed size array
    mutable_array[0] = 100; // modify an element
    println!("Fixed array: {:?}", fixed_size_array);
    println!("Mutable array after modification: {:?}", mutable_array);

    let array2: [i32; 5] = [0; 5]; // array with all elements initialized to 0
    println!("Array with default values: {:?}", array2);

    // vectors
    // unlike arrays, vectors can grow and shrink in size
    let mut vec: Vec<i32> = Vec::new(); // create an empty vector
    vec.push(1); // add elements
    vec.push(2);
    vec.push(3);
    println!("Vector after adding elements: {:?}", vec);

    let mut vec2: Vec<i32> = vec![10, 20, 30, 40]; // create a vector with initial values
    vec2.pop(); // remove the last element
    println!("Vector after popping an element: {:?}", vec2);

    // tuples
    // tuples can hold different types unlike arrays and vectors
    let info: (i32, &str, i32, f64, &str) = (42, "Tuple Example", 100, 3.14, "Tuple Example"); // tuple with different types
    println!("Tuple contents: {:?}", info);
    // accessing tuple elements
    println!("First element: {}", info.0);
    println!("Second element: {}", info.1);
    println!("Third element: {}", info.2);
    println!("Fourth element: {}", info.3);
    println!("Fifth element: {}", info.4);

    // destructuring tuples with value binding
    // let correct_info = ("Corrected Tuple", 100, 6.28, "Corrected Tuple");
    // println!("Corrected Tuple contents: {:?}", correct_info);

    //destructuring tuples
    // let (salary: i32, bonus: i64) = (50, 5000);
    // println!("Salary: {}, Bonus: {}", salary, bonus);


    // empty tuple
    // empty tuples are also known as unit type
    // empty tuples are used when there is no meaningful value to return
    // they are represented by (), there are no elements inside, the size is zero
    // they can be used in functions that do not return any value
    // they can also be used as a placeholder and to indicate the absence of a value
    let empty: () = (); // unit type, represents an empty value
    println!("Empty tuple: {:?}", empty);

    // functions that return empty tuples
    fn do_nothing() -> () {
        // this function does nothing and returns an empty tuple    
    }
    fn empty_return () {
        // this function also returns an empty tuple implicitly
    }

    // Functions
    // functions are used to encapsulate reusable code
    // they can take parameters and return values
    // functions help in organizing code and improving readability
    // functions are defined using the 'fn' keyword followed by the function name and parentheses
    // the function body is enclosed in curly braces
    // functions can have parameters with specified types
    // functions can return values using the '->' syntax followed by the return type    

    fn 
}