
// compound data types
//     -&str and String
//     -Arrays
//     -Vectors
//     -Tuples
//     -Empty Tuples


fn main() {
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
}
