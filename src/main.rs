mod learn;
// use std::collections::HashMap;
// use std::io;

fn main() {
    // student_record_system();
    learn::run();
}

// fn cal() {
//     println!("Enter first number:");
//     let mut number1 = String::new();
//     io::stdin().read_line(&mut number1).expect("number1");

//     println!("Choose operation: 1) +  2) -  3) *  4) ");
//     let mut choice = String::new();
//     io::stdin().read_line(&mut choice).expect("operation");

//     println!("Enter second number:");
//     let mut number2 = String::new();
//     io::stdin().read_line(&mut number2).expect("number2");
//     let n1: i32 = number1.trim().parse().expect("number 1");
//     let n2: i32 = number2.trim().parse().expect("number 2");

//     match choice.trim() {
//         "1" => println!("{}", n1 + n2),
//         "2" => println!("{}", n1 - n2),
//         "3" => println!("{}", n1 * n2),
//         "4" => println!("{}", n1 / n2),
//         _ => println!("Invalid operation"),
//     }
// }

// fn student_record_system() {
//     let mut student_records = HashMap::new();
//     student_records.insert(String::from("raj"), 20);

//     loop {
//         println!("Enter command 1)add 2)get 3)list 4)quit");
//         let mut input = String::new();
//         io::stdin().read_line(&mut input).expect("failed");
//         match input.trim() {
//             "add" => {
//                 println!("Enter name");
//                 let mut name = String::new();
//                 io::stdin().read_line(&mut name).expect("failed");
//                 println!("Enter age");
//                 let mut age = String::new();
//                 io::stdin().read_line(&mut age).expect("failed");
//                 let num_age: i32 = age.trim().parse().unwrap();
//                 student_records.insert(name.trim().to_string(), num_age);
//             }
//             "get" => {
//                 println!("Enter student name");
//                 let mut name = String::new();
//                 io::stdin().read_line(&mut name).expect("failed");
//                 match student_records.get(name.trim()) {
//                     Some(age) => println!("Age: {age}"),
//                     None => println!("Student not found"),
//                 }
//             }
//             "list" => {
//                 for (name, age) in &student_records {
//                     println!("{} {}", name, age);
//                 }
//             }
//             "quit" => break,
//             _ => println!("Invalid command"),
//         }
//     }
// }

// fn play_quiz() {
//     let questions = vec!["what is cpu", "what is ram", "what is rom"];
//     let ans = vec![
//         "central processing unit",
//         "random access memory",
//         "read only memory",
//     ];
//     let mut score: i32 = 0;
//     for i in 0..questions.len() {
//         println!("{}", questions[i]);
//         let mut user = String::new();
//         io::stdin().read_line(&mut user).expect("answer");
//         if user.trim().to_lowercase() == ans[i].to_lowercase() {
//             score += 1;
//         }
//     }
//     println!("Your final score: {}", score);
// }
