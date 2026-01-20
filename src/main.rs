
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
    println!("The modified value of y is: {}", y);
    
    //primintive data types review
    let num : i32 = 5;
    println!("Here is the number : {:?}", num);
    // string
    let fixed_str: &'static str = "String with fixed length"; // immutable, fixed size
    let mut growable_string: String = String::from("Growable String"); // mutable, dynamic size
    growable_string.push_str("- add to more content"); // add characters to the end
    println!("Here is a fixed string: {}", fixed_str);
    println!("this is the growable array : {}", growable_string);

    // arrays
    let fixed_array: [i32;5] = [1, 2, 3, 4, 5]; // fixed size array
    let mut matuble_array: [i32;3] = [10, 20, 30]; // mutable fixed size array
    matuble_array[0] = 100; // modify an element
    println!("Fixed array: {:?}", fixed_array);
    println!("Mutable array after modification: {:?}", matuble_array);

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
    let empty: () = (); // unit type, represents an empty value
    println!("Empty tuple: {:?}", empty);
}
