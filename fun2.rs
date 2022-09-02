fn main(){
    let mut res = 0.0;
    modifies(&mut res); // this will change the value of res to 1.0 
    println!("res is {}", res);
        
    
    // ! using type-after-variable for let
    let bigInt: i64 = 0; 
}

// this function takes in a mutable reference, allowing us to modify its argument
fn modifies(x: &mut f64){
    *x = 1.0
}

