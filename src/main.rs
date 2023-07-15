fn main() {
   println!("Hello, World!");
   let first_letter = 'A';
   let space = ' ';
   let broken_heart = '💔';

    // casting = simple type change using 'as'

    let my_number: u16 = 8;
    let second_number: u8 = 10;
    let third_number = my_number + second_number as u16;
    
    let my_number2 = 'a' as u8;
    println!("Hello, World! My number is {}", my_number2);
}
