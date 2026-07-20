use std::io;
use rand:: RngExt;
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
    guess_number();
              

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

fn guess_number (){
    println!("guess number between 1 and 100!");
    let secret: i32 = rand::rng().random_range(1..=100);
    loop{
    let mut input:String = String::new();
    io::stdin().read_line( &mut input).expect("number");
    let guess:i32 = input.trim().parse().expect("number to found!");
    println!("you guessed {}",guess);
    if guess<secret{
        println!("too low");
    }else if guess>secret{
        print!("too high");
    }else{
        print!("won!!!");
        break;
    }
    }




}





