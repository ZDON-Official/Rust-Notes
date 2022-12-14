/*
    ! More about vectors 
    Vec! is macro that can be used for initializing a vector
    * .push() - add values at the end 
    * .pop() - remove value from the end
    * .insert() - insert value at an arbitrary position 
    * .remove() - remove value from an arbitrary position
    * .clear() - empty the vector, make its size 0 
    * .sort() - sort a vector 
    * .dedup() - removes duplicates
*/

fn main(){
    let mut v1 = vec![10,20,30,40];
    v1.pop(); // will remove 40 from the vector

    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);

    assert_eq!(v1, v2);

    v2.extend(0..2);
    assert_eq!(v2, &[10, 20, 30, 0, 1]);
}