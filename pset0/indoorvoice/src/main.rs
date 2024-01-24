// Load neccesary packages
use std::io;

fn main() {
   // Create a new string
   let mut input = String::new();

   // Read a line of input from the user and handle potential errors
   let _ = io::stdin().read_line(&mut input); 
      
   let lowercase = input.to_lowercase();

   // Print the user's input
   println!("{}",lowercase);
}
