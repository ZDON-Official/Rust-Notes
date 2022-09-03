/*
    ! Strings - memoery is allocated dynamically and is resizeable, like C++
    * Rust has string ans &str(string slice)
*/
fn main(){
    let text = "Hello Dolly"; // this is a string slice
    let s = text.to_string(); // it is now an allocated string
    
    dump(text);
    dump(&s);

    // ! Just like a vector, you can 'push' and 'pop' one of the ends of string
    let mut s = String::new(); // new empty string
    s.push('H');
    s.push_str("ello");
    s.push(' ');
    s += "World!"; // this is short for 'push_str'
    
    s.pop(); // this will remove the last character
    assert_eq!(s, "Hello World");
}

fn dump(s : &str){ // string slice reference
    println!("str '{}'", s);
}