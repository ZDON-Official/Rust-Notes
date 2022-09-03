/*
    ! arrays are zero-indexed
    ! arrays are not mutable by default and have a fixed size
    * arrays are not used that often in Rust because the
    * type of an array also includes its size 
    ! instead slices are used - they behave very much like an array
    ! slice's length is not known at compile time, unlike arrays
*/

fn main(){
    let arr = [10,20,30,40]; // <------- this is an Array
    let first = arr[0]; // first element in the array

    println!("first {}", first);

    for i in 0..4{
        println!("[{}] = {}", i, arr[i]);
    }
    println!("length {}", arr.len()); // prints the length of the array

    // ! Slice
    let res = sum(&arr);
    println!("sum {}", res);

    /*
        ! Rust slices keep track of their size unlike C
        You will have to use '&' operator to pass an array as a slice 
        * '&' - is pronounced as 'borrow', while it is pronounced as 'address of' in C
     */
}

fn sum(values: &[i32])->i32{ // * <----- values is a reference to a slice
    let mut res = 0;
    for i in 0..values.len(){ // ! 0 to the length of the values slice
        res += values[i];
    }
    res
}


