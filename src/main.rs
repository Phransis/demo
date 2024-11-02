//Taking user input

// use std::io;
// fn main() {
//     println!("What's your name?");
//     let mut name=String::new();

//     io::stdin()
//     .read_line(&mut name)
//     .expect("Failed to read line");
//         println!("My name is {name}");
    
// }






//Guess game without generating random number

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






//Taking user input

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






//Guess game with a system generated number


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







//Guess game

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







//Tuples

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









//Looping till eternity

// fn main(){
//     loop{
//         println!("Im a winner")
//     }
// }


// use std::io;

// fn main() {
//     println!("Hello! Please enter a temperature in Celsius:");

//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");

//     // Convert the input to a floating-point number
//     let temperature: f32 = input.trim().parse().expect("Invalid input. Please enter a number.");

//     // Perform the temperature conversion (replace this with your conversion formula)
//     let after_conversion = temperature * 9.0 / 5.0 + 32.0;

//     println!("{} degrees Celsius is equivalent to {} degrees Fahrenheit", temperature, after_conversion);
// }


// fn main(){
//     let mut s = String::from("Hello");
//     s.push_str(", world");
//     println!("{}",s)
//     println!("code working")
// }


// fn main(){
//     let name = "Handla";
//     let age = 100;
//     let weight = 50.67;
//     let male = true;
//     let x = five();
//         println!("My name is {name} and I'm {age} years old and a {male}. I weight {weight}kg. {x}");
//         write_name("Duo");
//         }
        
//         fn write_name(your_name: &str){
//             println!("Its all good, {your_name}")
//             }
            
//             fn five() -> i32 {
//                 5
//                 }
           
           
// // Ownnership
//     fn main(){
//         let mut s = String::from("hello ");
//         s.push_str("handla");
//         println!("{s}");
//         println!("{}", s);
//         let s1 = String::from("hello");
//         println!("{}, Cassandra", s1);
//         let s2 = s1;
    
//         println!(" {}, world!", s2);
    
//     }



// Slice
// fn main() {
//     let arr = [1, 2, 3, 4, 5];

//     let slice1 = &arr[1..4];
//     let slice2 = &arr[2..];
//     let slice3 = &arr[..3];
//     let slice4 = &arr[..];

//     println!("slice1: {:?}", slice1);
//     println!("slice2: {:?}", slice2);
//     println!("slice3: {:?}", slice3);
//     println!("slice4: {:?}", slice4);
    
//     let result: i32 = sum(&arr[1..5]);
//     println!("Result: {result}")
// }
// fn sum(slice: &[i32]) -> i32{
//     slice.iter().sum()
// }






// Stucts
// fn main(){
//     #[derive(Debug)]
//     struct User{
//         id: u128,
//         firstname: String,
//         lastname: String,
//         email: String,
//         active: bool,
//     }
//     let  user1 = User{
//         id: 1,
//         firstname: String::from("Handla"),
//         lastname: String::from("Duo"),
//         email:String::from("test@mail.com"),
//         active: true,
//     };
//     println!("{user1:?}");

    
   
   
    // update_user(Stacy, Brown, bstacymailcom);
//     fn update_user(firstname: String, lastname:String, email: String) -> User{
//         User{
//             id: 2,
//             firstname,
//             lastname,
//             email,
//             active: false
//         }

//     }

//    let update = update_user("Stacy".to_string(), "Brown".to_string(), "bstacymailcom".to_string());
//     println!("Update: {update:?}");


    
    
    //Creating instances from other instances
    // let user2 = User{
    //     id: 3,
    //     firstname: user1.firstname,
    //     lastname: user1.lastname,
    //     email: String::from("mail@test.com"),
    //     active: true
    // };
    // println!("{user2:?}");

  
  
   
   //Using struct update syntax
    // let user3 = User{
    //     email: String::from("new@mail.com"),
    //     ..user2
    // };
    // println!("{user3:?}");


   
   
    //Tuple struct
    // #[derive(Debug)]
    // struct Color(i128, i128, i128);
    // struct Point(i128, i128, i128);
    
    // let black=Color (0, 0, 0);
    // let origin= Point(0, 0, 0);

   
   
   //Example
//    #[derive(Debug)]
//     struct Rectangle{
//          length: i128,
//         breadth: i128,
//     }
    // area_rectangle(10, 5);
//     let rect1 = Rectangle{
//         length: 30,
//         breadth: 15
//     };
//     println!("RECT1{:#?}",rect1 );
//     println!("The area is {:#?}",area_rectangle(100, 50) )
    
// }
// fn area_rectangle(lenght: i128, breadth: i128) -> i128{
//     lenght * breadth
// }


// struct Rectangle{
//     height: u32,
//     width: u32,
// }

// struct Circle{
//     pi: f32,
//     radius: f32
// }

// impl Circle {
//     fn area_circle(&self) -> f32{
//         self.pi * (self.radius*self.radius)
//     }
// }

// impl Rectangle {
//     fn area_rectangle(&self) -> u32 {
//         self.height * self.width
//     }
// }


// fn main(){
//     let rect1 = Rectangle{
//         height : 50,
//         width : 100,
//     };

//     println!("The area of a Rectangle with height {} and width {} is {}", rect1.height, rect1.width, rect1.area_rectangle());

//     let cir1 = Circle{
//         pi: 3.142,
//         radius: 33.3,
//     };
//     println!("The area of a Circle with pi {} and radius {} is {}", cir1.pi, cir1.radius, cir1.area_circle())
// }


// use std::io;
// fn main(){
//     changes("Handla".to_string(), "Duo".to_string());

//     fn changes(name:String, lname:String) {
//         println!("What's your name?");

//         let  name = String::new();
//         let  lname= String::new();

//         println!("the names are {} {}", name, lname)
//     }
// }


//ENUMS

// fn main() {
//     enum Coin {
//         Penny,
//         Nickel,
//         Dime,
//         Quater,
//     }

//     fn value_in_cents(coin: Coin) -> u8 {
//         match coin {
//             Coin::Penny => 1,
//             Coin::Nickel => 5,
//             Coin::Dime => 10,
//             Coin::Quater => 25,
//         }
//     }
// }


//MATCH AND ENUMS

// fn main() {


//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             None => None,
//             Some(i) => Some(i +1),
//         }
//     }

//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     println!("The Some function: {:?}", plus_one(six));
//     println!("The None function: {:?}", plus_one(none));
    
// }


// VECTORS
// fn main() {

//     let v1 = vec![100];
//     println!("Integer Vector: {:?}", v1);

//     let v2 = vec!["Hello Handla"];
//     println!("String Vector: {:?}", v2);
    
//     let mut vector = Vec::new();
//     vector.push(4);
//     vector.push(6);
//     vector.push(7);

//     println!("Mutabale vector: {:?}", vector);

//     let mut v = vec![1, 2, 3];
//     v.push(3);
//     v.push(2);
//     v.push(1);
//     let ve = vec![4,5,6];
//     println!("Vector v: {:?}", v);


    //Adding and Removing elements in a Vector
//     let mut vector = vec![v, ve];
    
//     vector.push([10,20,30].to_vec());
//     println!("Vector-Push: {:?}", vector);
    
//     vector.pop();
//     println!("Vector-Pop: {:?}", vector);

// }


//HASHMAPS
// use std::collections::HashMap;

// fn main() {
//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);

//     for (key, value) in &scores{
//         println!("{key} {value}");
//     }

//     let team_name = String::from("Blue");
//     let score = &team_name;

//     println!("Score is {:?}", score);
// }

#![no_std]
use soroban_sdk::{contract,contracttype, contractimpl, Env, Symbol,symbol_short, log};

#[contracttype]
#[derive(Clone)]
pub struct Poll{
    pub yes: u64,
    pub no: u64,
    pub total: u64,
}

const POLL: Symbol = symbol_short!("POLL");

#[contracttype]
pub enum Registry{
    Record(Symbol)
}

#[contracttype]
#[derive(Clone)]
pub struct Record{
    pub name: Symbol,
    pub selected: Symbol,
    pub votes: u64,
    pub time: u64,
}

const YES: Symbol = symbol_short!("yes");
// const No: Symbol = symbol_short!("no");


#[contract]
pub struct VoteContract;

#[contractimpl]
impl VoteContract {

    pub fn register_votes(env: Env, user: Symbol, votes: u64, selected: Symbol) {
        let mut records = Self::view_voter(env.clone(), user.clone());
        let time = env.ledger().timestamp();

        if votes ==0 || records.time != 0 || votes > 5 {
            panic!("cannot vote");
        }
        else {
            let mut poll = Self::view_poll(env.clone());
            records.name = user.clone();
            records.selected = selected;
            records.votes = votes;
            records.time = time;

            if records.selected == YES {
                poll.yes += votes;
            }
            else {
                poll.no += votes;
            }
            poll.total += votes;

            env.storage().persistent().set(&Registry::Record(user), &records);
            env.storage().persistent().set(&POLL, &poll);
            log!(&env, "Votes Registered");
            return;
        }
    }

    pub fn view_poll(env:Env) -> Poll {
        env.storage().instance().get(&POLL).unwrap_or(Poll{
            yes: 0,
            no: 0,
            total: 0,
        })
    }
    pub fn view_voter(env:Env, voter: Symbol) -> Record {
        let key = Registry::Record(voter.clone());
        env.storage().instance().get(&key).unwrap_or(Record{
             name: symbol_short!("not_found"),
             selected: symbol_short!("none"),
             votes: 0,
             time: 0,
        })
    }
}