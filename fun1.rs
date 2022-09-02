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

    // ! pass by reference 
    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("{} {}", res1, res2);
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

// ! this is an example of a recursive function using no return expression style
fn factorial(n: u64)->u64{
    if n == 0 {
        1
    } else {
        n * factorial(n-1)
    }
}

// ! passing parameters by reference, & is used a to reference a value
fn by_ref(x: &i32)->i32{
    *x + 1 //! * is used to dereference a value
}