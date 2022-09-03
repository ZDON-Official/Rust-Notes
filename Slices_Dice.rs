/*
    ! Slicing and Dicing
    {} cannot be used to print an Array, instead debug print is used
    Arrays can only contain values of ONE TYPE <-----------------------
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

    // ! slice
    let slice1 = &ints[0..2]; // only the first to not including index 2
    let slice2 = &ints[1..]; // from index 1 till end
    
    println!("ints {:?}", ints);
    println!("slice1 {:?}", slice1);
    println!("slice2 {:?}", slice2);
    

} 