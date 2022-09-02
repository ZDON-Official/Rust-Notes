/*
    ! Functions types are explicit
    this means that the compiler will not work out the type for you
*/

// type is explicitly defined 
fn sqr(x: f64)->f64{
    //return x*x;
    x*x // ! most function are writtern without a return statement 
    // ! because the last value in a function become its ecpression/ return value
}
  
fn main(){
    let res = sqr(2.0);
    println!("square is {}", res);
}

// ! another example of no return expression style
fn abs(x: f64)->f64{
    if x > 0.0{
        x
    } else {
        -x
    }
}

//* Functions can still be written with a 'return' at the end, but the code is cleaner 
//* without it