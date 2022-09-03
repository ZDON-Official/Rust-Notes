/*
    ! Vectors - re-sizeable ararys ans beahve much like Python list and C++ Vectors
*/

fn main(){
    let mut v = Vec::new(); // create a new vector
    v.push(10); // push value at the end of the vector 
    v.push(20);
    v.push(30);

    let first = v[0]; // will panic if out of range
    let maybe_first = v.get(0);

    println!("v is {:?}", v); // {:?} is debug print and is used for printing arrays
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);
}