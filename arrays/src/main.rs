fn main() {

    // arrays
    let arr = [1,2,4];
    println!("{:?}", arr);

    let mut i = 0;

    loop {
        if i == arr.len(){
            break;
        }
        println!("{}", arr[i]);
        i = i+1;
    }

    // tuples

    // let tuple = ("Praash", 19, "Varma");
    // println!("{}", tuple.0);
    // println!("{}", tuple.1);
    // println!("{}", tuple.2);
    // println!("{:?}", tuple);


    // Arrays and Slice

    let arr = [0,1,2, 3, 4];
    let slice = &arr[1..3];
    println!("{:?}", arr);
    println!("{:?}", slice);


}
