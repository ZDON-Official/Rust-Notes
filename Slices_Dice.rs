/*
    ! Slicing and Dicing
    {} cannot be used to print an Array, instead debug print is used
    Arrays can only contain values of // ! ONE TYPE
*/

fn main(){
    // * Debug print {:?}
    let ints = [1,2,3,4,5];
    let floats = [1.1,2.1,3.1];
    let strings = ["hello", "world"];
    let ints_ints = [[1,2],[10,20]]; // ! Matrix
    println!("ints {:?}", ints);
    println!("floats {:?}", floats);
    println!("strings {:?}", strings);
    println!("ints_ints {:?}", ints_ints);
    
    println!("");

    // ! Slicing and Dicing
    let slice1 = &ints[0..2]; // only the first to not including index 2
    let slice2 = &ints[1..]; // from index 1 till end
    
    println!("ints {:?}", ints);
    println!("slice1 {:?}", slice1);
    println!("slice2 {:?}", slice2);
    println!("");

    // this is similar to python slices, except a copy is never made
    // rather, the slices all borrow their data from their arrays

    // ! Optional Values
    // there is no try-catch block to wrap code that may panic, instead 
    // Rust has a 'get' method for slices that does not panic

    let ints1 = [1,2,3,4,5];
    let slice_get = &ints; // a slice of the array ints1
    let first = slice_get.get(0); // get the first element 
    let last = slice_get.get(5); // ! this line return none since 5 is out of bound 

    println!("first {:?}", first); // some(1)
    println!("last {:?}", last); // none

    // ! 'Option' type is either 'some' or 'none' 

    // ! Option type methods 
    println!("first {} {}", first.is_some(), first.is_none()); // first 'true false'
    println!("last {} {}", last.is_some(), last.is_none()); // 'last false true'
    println!("first value {}", first.unwrap()); // 'first value 1'
    
    // * 'unwrapping' 'none' will panic, but '.is_some' can be used to make sure the value
    // * is not 'none'
} 