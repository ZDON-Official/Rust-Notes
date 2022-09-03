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
}