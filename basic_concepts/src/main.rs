fn main() {
    // Variables and Mutability

    // by default vars are immutable in rust
    // let x = 5;
    // println!("x :{x}");
    // x = 5;  can't do coz x is not mutable

    // that's how we can make mutable vars
    // let mut x = 5;
    // println!("x : {x}");
    // x =  1314;
    // println!("x : {x}");


    // Constants
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // print!("{THREE_HOURS_IN_SECONDS}")

    // Shadowing 
    // Shadowing in Rust is a feature that allows you to declare a new variable with the same name as a previous variable. 
    // When you declare a variable with the same name as a previously declared variable, the new variable shadows the old one. This means that the old variable is no longer accessible through its name, and any subsequent references to the variable name will refer to the new variable.
    // let x: i32 = 5;

    // let x: i32 = x + 1;

    // {
    //     let x: i32 = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");



    // Compound Types
    // let tup = ("Praash", 100);
    // let (x,y) = tup;
    // println!("{}", y);
    // println!("{}", x);
    // println!("Tup: {:?}", tup);
}
