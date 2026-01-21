
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
    // the naming convention for function is snake case 
    // parameters in function are also defined like variable

    // difference between expression and statement
    // statement is an instruction that performs an action but does not return a value
    // expression is a piece of code that evaluates to a value

    fn add(num1:i32, num2:i32) -> i32 {
        num1 + num2 // expression that returns the sum
    }

    fn main() {
        add(num1=5, num2=10); // calling the add function
    }

    // code blocks
    // code blocks are used to group multiple statements and expressions together
    // they are enclosed in curly braces {}
    // code blocks can be used to create scopes for variables
    // the last expression in a code block is the return value of the block
    let result: i32 = {
        let a: i32 = 10;
        let b: i32 = 20;
        a + b // last expression, the result of the block
    };
    println!("Result from code block: {}", result);

   // control flow
   // control flow statements are used to control the execution of code based on conditions
   // common control flow statements include if, else if, else, and match
   let number: i32 = 15;
   if number < 10 {
       println!("The number is less than 10");
   } else if number == 10 {
       println!("The number is equal to 10");
   } else {
       println!("The number is greater than 10");
   } 

   // match control flow
   let day: u32 = 3;
   let day_name: &str = match day {
       1 => "Monday",
       2 => "Tuesday",
       3 => "Wednesday",
       4 => "Thursday",
       5 => "Friday",
       6 => "Saturday",
       7 => "Sunday",
       _ => "Invalid day", // wildcard pattern
   };
   println!("The day is: {}", day_name);

    // loops
    // loops are used to repeat a block of code multiple times
    // common loop types include loop, while, and for
    // infinite loop using loop keyword
    let mut count: i32 = 0;
    loop {
        count += 1;
        if count == 5 {
            break; // exit the loop when count reaches 5
        }
    }
    println!("Count after loop: {}", count);

    // while loop
    // the while loop continues as long as the condition is true
    // here we use while loop to count down from 5 to 1
    // when the count reaches 0, the loop stops
    // this is useful for scenarios where the number of iterations is not known beforehand
    let mut countdown: i32 = 5;
    while countdown > 0 {
        println!("Countdown: {}", countdown);
        countdown -= 1; // decrement the countdown
    }
    println!("Liftoff!");

    let mut num: i32 = 0;
    while num < 5 {
        println!("Number in while loop: {}", num);
        num += 1;
    }

    // for loop
    // the for loop is used to iterate over a range or collection
    // here we use a for loop to iterate over a range of numbers from 0 to 4
    // the for loop is useful when the number of iterations is known beforehand
    for i in 0..5 {
        println!("Number in for loop: {}", i);
    }

    // iterating over an array using for loop
    let arr: [i32; 3] = [10, 20, 30];
    for element in arr.iter() {
        println!("Array element: {}", element);
    }

    // iterating over a vector using for loop
    let vec: Vec<i32> = vec![100, 200, 300];
    for value in vec.iter() {
        println!("Vector value: {}", value);
    }

    // using enumerate to get index and value
    let fruits: Vec<&str> = vec!["Apple", "Banana", "Cherry"];
    for (index, fruit) in fruits.iter().enumerate() {
        println!("Fruit {}: {}", index, fruit); 
    }

    // using loop labels to break from nested loops
    'outer: for i in 1..4 {
        for j in 1..4 {
            if i * j == 4 {
                break 'outer; // break from the outer loop
                break; // break from the inner loop
            }
        }
    }
    println!("Exited nested loops using loop label");

    // using continue to skip an iteration
    for num in 1..6 {
        if num % 2 == 0 {
            continue; // skip even numbers
            println!("Number in continue loop: {}", num);
        }
    }
    println!("Finished continue loop");

    // mutation in function parameters
    // by default, function parameters are immutable
    // to modify a parameter, you need to use the 'mut' keyword
    fn increment(mut value: i32) -> i32 {
        value += 1; // modify the parameter
        value // return the modified value
    }
    let original: i32 = 10;
    let incremented: i32 = increment(original);
    println!("Original value: {}, Incremented value: {}", original, incremented);

    //conditionals
    // conditionals are used to execute code based on certain conditions
    let age: u32 = 18;
    if age >= 18 {
        println!("You are an adult.");
    } else {
        println!("You are a minor.");
    }

    // common sytax for loop and ranges
    // ranges are used to define a sequence of numbers
    
    for i in 0..5 { // exclusive range
        println!("Exclusive range value: {}", i); 
    }

    // compiler directives
    // compiler directives are special instructions that affect the compilation process
    // they are usually specified using attributes like #[attribute]
    
    // here is an example of a compiler directive to allow dead code
    #[allow(dead_code)] // this directive tells the compiler to ignore dead code warnings
    fn unused_function() {
        // this function is not used anywhere
    }
    // the unused_function will not generate a warning due to the directive above
    // this is useful during development when you have code that is not yet used
    // but you want to keep it for future use
    // remember to remove such directives in production code to maintain code quality
    // always use compiler directives judiciously to avoid hiding important warnings
    // use them only when necessary and understand their implications
    // improper use of compiler directives can lead to overlooked issues in the code
    // always strive for clean and warning-free code


    // variable conventions and statis
    // variable names should be descriptive and use snake_case
    // static variables have a fixed memory location and live for the entire duration of the program
    // static variables are useful for constants that are shared across multiple functions or modules
    // static variables are declared with the 'static' keyword
    
    static GLOBAL_COUNT: i32 = 100; // static variable declaration
    println!("Global count from static variable: {}", GLOBAL_COUNT);

    // operators
    // operators are symbols that perform operations on variables and values
    // common operators include arithmetic, comparison, and logical operators
    // -arithmetic operators
    // - comparison operators
    // - logical operators  
    let sum: i32 = 5 + 10; // addition
    let difference: i32 = 10 - 5; // subtraction
    let product: i32 = 5 * 10; // multiplication
    let quotient: i32 = 10 / 5; // division
    let remainder: i32 = 10 % 3; // modulus
    println!("Sum: {}, Difference: {}, Product: {}, Quotient: {}, Remainder: {}", sum, difference, product, quotient, remainder);
    
    // comparison operators
    // comparison operators are used to compare two values
    // they return a boolean value (true or false)
    let is_equal: bool = 5 == 5; // equality
    let is_not_equal: bool = 5 != 10; // inequality
    let is_greater: bool = 10 > 5; // greater than
    let is_less: bool = 5 < 10; // less than
    let is_greater_equal: bool = 10 >= 10; // greater than or equal to
    let is_less_equal: bool = 5 <= 10; // less than or equal to
    println!("Is Equal: {}, Is Not Equal: {}, Is Greater: {}, Is Less: {}, Is Greater Equal: {}, Is Less Equal: {}", is_equal, is_not_equal, is_greater, is_less, is_greater_equal, is_less_equal);

    // loogical operators
    // logical operators are used to combine boolean values
    let and_result: bool = true && false; // logical AND
    let or_result: bool = true || false; // logical OR
    let not_result: bool = !true; // logical NOT
    println!("AND Result: {}, OR Result: {}, NOT Result: {}", and_result, or_result, not_result);

    // loop and ranges
    // loops are used to repeat a block of code multiple times
    // ranges are used to define a sequence of numbers
    for i in 1..=5 { // inclusive range
        println!("Inclusive range value: {}", i);   
    }
    // the loop will print numbers from 1 to 5 inclusive
    // the '..=' syntax creates an inclusive range that includes the end value
    // this is useful when you want to include the last number in the sequence
    // remember to choose the appropriate range syntax based on your requirements
    // using ranges effectively can simplify your code and improve readability
    // always consider edge cases when working with ranges and loops
    // test your loops to ensure they behave as expected
    // avoid infinite loops by ensuring the loop condition will eventually be false
    // use loops and ranges judiciously to enhance code clarity and maintainability
    // practice writing loops and ranges to become proficient in their usage

    // reverse ranges
    for i in (1..=5).rev() { // reverse inclusive range
        println!("Reverse range value: {}", i);
    }

    // iterating with a step size
    // here we use the step_by method to iterate over a range with a step size of 2
    for i in (0..10).step_by(2) {
        println!("Step by 2 value: {}", i);
    }

    // varaible conventions and statics
    // variable names should be descriptive and use snake_case
    let user_name: &str = "Alice"; // descriptive variable name
    println!("User name: {}", user_name);

    // static variable example
    static MAX_USERS: u32 = 1000; // static variable declaration
    println!("Maximum users allowed: {}", MAX_USERS);

    // compiler directives
    // compiler directives are special instructions that affect the compilation process
    // they are usually specified using attributes like #[attribute]
    #[allow(unused_variables)] // this directive tells the compiler to ignore unused variable warnings
    let unused_var: i32 = 42; // this variable is not used anywhere
    // the unused_var will not generate a warning due to the directive above
    // this is useful during development when you have code that is not yet used
    // but you want to keep it for future use
    // remember to remove such directives in production code to maintain code quality
    // always use compiler directives judiciously to avoid hiding important warnings
    // use them only when necessary and understand their implications
    // improper use of compiler directives can lead to overlooked issues in the code
    // always strive for clean and warning-free code

}