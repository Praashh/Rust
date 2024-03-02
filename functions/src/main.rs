fn main() {
    // hello_world();
    let mut x = 32;
    let mut y = 432;

    let mut res = sum(x, y);
    println!("{}", res);
}
// fn hello_world(){
//     println!("Hello World!");
// }

fn sum (x:i32, y:i32) -> i32{
    return x+y;
}