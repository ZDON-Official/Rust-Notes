/*
    ! Iterators - an Object with a 'next()' method which returns an 'Option'
*/
fn main(){
    let mut iter = 0..3;
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);

    // ! looping over an iterator
    /*
        let arr = [10, 20, 30];
        for i in arr {
            println!("{}", i);
        }
        ! this is attempting to iterate over an array, which will fail
    */
    // ! this however will work
    let arr = [10, 20, 30];
    let slice = &arr;
    for i in slice{
        println!("{}", i)
    }
    // * this method is more efficient than - 'for i in 0..slice.len() {}'

    /*
        A better way of doing sum in Rust
        ! Types need to be explicit in this case because Rust does not have 
        ! information 
    */
    let sum: i32 = (0..5).sum();
    println!("sum was {}", sum);
    
    let sum: i64 = [10,20,30].iter().sum();
    println!("sum was {}", sum);

    // ! Windows method
    let ints = [1,2,3,4,5];
    let slice = &ints;

    // this will print 2 elements at a time
    for s in slice.windows(2){
        println!("window {:?}", s);
    } 
    
    // ! Chunks method - does not overlap
    for s in slice.chunks(2){
        println!("chunks {:?}", s);
    }
}