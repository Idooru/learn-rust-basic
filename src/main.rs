fn main() {
   let my_number = 9_u8;
   let other_number = 1_000_000_000u64;

   let my_number = 9.6;
   let other_number = 9;
   println!("{}", my_number as i32 + other_number);
   println!("{}", my_number + other_number as f64);
}
