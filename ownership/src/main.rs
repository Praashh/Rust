fn main() {

    // Ownership Rules
    // Each value in Rust has an owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.


    // Variable Scoping and Memory ( Stack and Heap )
    
    // {
        // let  s : &str = "Praash"; // allocating the value in stack
        // let s : String = String::from("Praash"); // allocating the value in heap
        // println!("{}", s);
    // }
    // println!("{}", s); // rust dropped the value of s;


    // let s1 : String = String::from("Praash");
    // let s2 : String = s1;  // Move ( Not Shallow Copy)
    // let s2 : String = s1.clone();

    // println!("{}", s1);



    // OwnerShip

    // let s = String::from("hello");
    // takes_ownership(s);
    // // println!("{}", s);

    // let i = 4234;
    // makes_copy(i);



    //  References

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of {} is {}", s1, len);


}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

fn calculate_length(s : String) -> (String, usize){
    let len = s.len();
    (s, len)
}