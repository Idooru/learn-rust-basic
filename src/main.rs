use std::mem::size_of;

fn main() {
    // char 4 bytes
    println!("Size of a char: {} bytes", std::mem::size_of::<char>());

    println!("Size of string containing 'a': {}", "a".len());  // ascii code
    println!("Size of string containing 'ㄱ': {}", "ㄱ".len());   // uni code
    println!("Size of string containing 'ㄱㄱㄱ': {}", "ㄱㄱㄱ".len());   // uni code

    let slice = "Hello!";
    println!("Slice is {} bytes and also {} characters", slice.len(), slice.chars().count());
    let slice2= "안녕!";
    println!("Slice is {} bytes and also {} characters", slice2.len(), slice2.chars().count());
}
