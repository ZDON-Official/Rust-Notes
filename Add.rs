fn main(){
    let mut sum = 0; // <- mut make a variable mutable
    for i in 0..5{
        sum += i;
    }

    println!("the sum is {}", sum);

}