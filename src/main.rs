// use std::io;
// fn main() {
//     println!("What's your name?");
//     let mut name=String::new();

//     io::stdin()
//     .read_line(&mut name)
//     .expect("Failed to read line");
//         println!("My name is {name}");
    
// }

// use std::io;

// fn main(){
//     println!("It's time for some guess game");
//     println!("Enter your guess game number");


//     let mut guess = String::new();
//     let happy_face = '\u{1F600}';
//     let sad_face = '\u{1F610}';


    
//         io::stdin()
//         .read_line(&mut guess)
//         .expect("Error");
//     println!("Your guess number is {guess}");
//     if guess.trim() == "1"{
//         println!("Congratulations{happy_face}...You are right");
//     }
//     else{
//         println!("Sorry try again{sad_face}...You lost");
//     }
// }

// use std::io;

// fn main() {
//     println!("What's your first name?");
//     let mut fname = String::new(); // Removed the type annotation, as it's not necessary.

//         // Added the .expect() method to handle potential errors in reading user input.
//         io::stdin()
//         .read_line(&mut fname)
//         .expect("Failed to read line");

//     println!("What's your surname?");
//     let mut sname = String::new();

//     io::stdin()
//     .read_line(&mut sname)
//     .expect("Failed to read line");

//     println!("What's your name?");
//     let mut age = String::new();


//     io::stdin()
//     .read_line(&mut age)
//     .expect("Failed to read line");

//     // Corrected the string interpolation by using {} and specifying the variable name.
//     println!("My first name is {}", fname);
//     println!("My Surname is {}", sname);
//     println!("I am {}", age);
// }

// use rand::Rng;
// use std::io;
// fn main() {
//     // Create a thread-local random number generator
//     let mut rng = rand::thread_rng();
//     let mut fname = String::new();

//     // Generate a random integer between 1 and 100 (inclusive)
//     let random_number = rng.gen_range(1..=100);
//         io::stdin()
//      .read_line(&mut fname)
//     .expect("Failed to read line");



//     println!("Random number: {}", random_number);
// }


// use std::io;
// use rand::Rng;

// fn main() {
//     println!("Guess the number!");



//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     println!("You guessed: {guess}");

//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     println!("The secret number is: {secret_number}");
// }

// fn main(){
//     let x = 45/40;
//     let y = 45%40;
//     let me = (67,"ok",45.7,true);
//     println!("{}{} {:?}",x,y,me);
// }


//Functions
// fn main(){
//     println!("Hello ");
//     second_function();
//     third_function();
//     parsing_func(22);
//     lets_go();
    
// }

// fn second_function(){
//     println!("Handla")
// }

// fn third_function(){
//     println!("Como estas?")
// }

// fn parsing_func(x:i32){
//     println!("You're {x} years ")
// }

// fn lets_go(){
//     let y ={
//         let z =2000;
//         let x = 22;
//         z-x +2000 + 12

//     };
//     println!("The value of y is {y}")
// }


//Using if in a let statement

// fn main(){
//     let condition = true;
//     let number = if condition {22} else {12};
//     println!("The value is {number}")
// }


//Loops

fn main(){
    loop{
        println!("Im a winner")
    }
}