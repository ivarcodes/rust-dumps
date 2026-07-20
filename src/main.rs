use std::io::{self, stdin};
#[warn(unused_must_use)]
fn main() {
    
 
    // let res = add(10, 20);
    // print!("{}",res);
    // let age = 15;
    // if age<13{
    //     print!("You are teenager");
    // }else if age<18 && age<13 {
    //     print!("you are kid");
    // }else {
    //     print!("adult");
    // }


    // loops
    // for i in 1..=5{
    //     print!("{}",i);
    // }

    // for i in 1..=10{
    //     if i%2==0{
    //         print!("{}",i);
    //     }
    // }

    //math table

    // for i in 1..=10{
    //     println!(" 5 *{} = {}",i,5*i);
    // }
    // let res = is_even(10);
    // println!("{}",res);


    // vectors 

    // let mut fruites = vec!["apple","banna","cherrry"];
    // for fruit in &fruites{
    //     println!("{}",fruit);
    // }
    // fruites.push("mango");
    // print!("{}",fruites[0]);
    // print!("{}",fruites.len());

    // let name = String::from("codeGuy");
    // greet(name);
    // guess_number();
    // cal();
    play_quiz();
              

}


// fn add(a:i32,b:i32)->i32{
//     return a+b;
// }

// fn is_even(num:i32)->bool{
//     if num%2==0{
//         return true;
//     }

//     return false;
// }

// fn greet(person:&String){
//     println!("Hi {} welcome",person) };

// fn guess_number (){
//     println!("guess number between 1 and 100!");
//     let secret: i32 = rand::rng().random_range(1..=100);
//     loop{
//     let mut input:String = String::new();
//     io::stdin().read_line( &mut input).expect("number");
//     let guess:i32 = input.trim().parse().expect("number to found!");
//     println!("you guessed {}",guess);
//     if guess<secret{
//         println!("too low");
//     }else if guess>secret{
//         print!("too high");
//     }else{
//         print!("won!!!");
//         break;
//     }
//     }




// }



fn cal(){
    println!("Enter first number:");
    let mut number1 = String::new();
    io::stdin().read_line(&mut number1).expect("number1");

    println!("Choose operation: 1) +  2) -  3) *  4) ");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("operation");

    println!("Enter second number:");
    let mut number2 = String::new();
    io::stdin().read_line(&mut number2).expect("number2");
    let n1:i32 = number1.trim().parse().expect("number 1");
    let n2:i32 = number2.trim().parse().expect("number 2");
    
    match choice.trim() {
        "1" =>println!("{}",n1+n2),
        "2" => println!("{}",n1-n2),
        "3" => println!("{}",n1*n2),
        "4" => println!("{}",n1/n2),
        _ => println!("Invalid operation"),
    }
}

fn play_quiz (){
    //storing questions and answers
let questions = vec!["what is cpu","what is ram","what is rom"];
let ans =vec!["central processing unit","random access memory","read only memory"];
    let mut score:i32 = 0;
    // asking questions
    for i in 0..questions.len(){
       
        println!("{}",questions[i]);
        let mut user = String::new();
        io::stdin().read_line(&mut user).expect("answer");
        if user.trim().to_lowercase()==ans[i].to_lowercase(){
            score += 1;
        }

    }
    println!("Your final score: {}", score);
}