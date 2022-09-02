fn main(){
    // ! loops 

    for i in 0..5{
        // this will loop 5 times, range is not inclusive like other languages
        println!("Hello {}", i);
    }

    // ! loops and if-statements
    for i in 0..5{
        if i % 2 == 0{
            println!("even {}", i)
        } else{
            println!("odd {}", i)
        }
    }

    // this does the same this as the for loop above
    for i in 0..5{
        let even_odd = if i % 2 == 0 {"even"} else {"odd"};
        println!("{} {}", even_odd, i);
    }

}