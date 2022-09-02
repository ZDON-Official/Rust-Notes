fn main(){
    let mut sum = 0; // <- mut make a variable mutable
    for i in 0..5{
        sum += i;
    }

    println!("the sum is {}", sum);
    
    let mut sum = 0.0; // <- for floats
    for i in 0..5{
        sum += i as f64; // this tell rust to convert int to float
    }

    println!("the sum is {}", sum);
}