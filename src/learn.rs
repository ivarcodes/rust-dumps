
pub fn run() {
    // your code here
    // struct  User { 
    //     username:String,
    //     email:String,
    //     active:bool
    // }

    // let user = User {
    //     username:String::from("codeGuy"),
    //     email:String::from("codeguy@gmail.com"),
    //     active:false
    // };
   
    // print!("{}",user.email);
    // println!("{}",user.username);
    // println!("{}",user.active);

    // struct Book {
    //     title:String,
    //     author:String,
    //     pages:i32
    // }

    // let book = Book {
    //     title:String::from("book1"),
    //     author:String::from("codeGuy"),
    //     pages:32
    // };

    // //enums and pattern matching

    // enum Direction {
    //     Up,
    //     Down,
    //     Left,
    //     Right
    // }

    // fn move_player(dir:Direction){
    //     match dir {
    //         Direction::Up=>println!("Moving up"),
    //         Direction::Down=>println!("Moving Down"),
    //         Direction::Left=>println!("Moving Left"),
    //         Direction::Right=>println!("Moving Right")
    //     }
    // }
    // enum Status {
    //     Sucess,
    //     Error(String),
    //     Loading
    // }

    // fn print_status(status:Status){
    
    // match  status {
    //     Status::Sucess=>println!("success"),
    //     Status::Error(msg)=>println!("Error: {}",msg),
    //     Status::Loading=>println!("false")
        
    // }
    // }

    // String vs &str

    // let s1:String = String::from("hello");
    // let mut s2 = String::new();
    // s2.push_str("code");
    // s2.push_str("guy");
    // let name = "codeguy";

    // let name1:&str = "hello";

   

//     fn  say_hello(name:&str){
//         print!("{}",name);
//     }

    
//   struct Person {
//         name:String,
//     }

//     let p  = Person{name:String::from("hello")};

//     let mut scores = HashMap::new();
//     scores.insert(String::from("blue"), 10);
//     scores.insert(String::from("red"), 20);
    
//     println!("{:?}",scores);
    

// let mut students = HashMap::new();
// students.insert(String::from("raj"), 20);
// students.insert(String::from("codeGuy"), 24);
// students.insert(String::from("rohit"), 18);

// println!("{:?}",students.get("rohit"));
// println!("{:?}",students.get("raj"));

// let n ="42".parse::<i32>().expect("not an number");
// println!("{}",n);


// trait Vehicle {
//     fn top_sepeed(&self)->u32;
//     fn vehicle_type(&self)->&str;
// }

// struct Bike {
//     name: String,
//     speed: u32,
//     model_name: String,
// }

// impl Vehicle for Bike {
//     fn top_sepeed(&self) -> u32 {
//         self.speed
//     }
//     fn vehicle_type(&self) -> &str {
//         &self.name
//     }
// }
#[derive(Debug,Clone,PartialEq)]
struct Bike {
    name:String,
    speed:u32,
    model_name:String,
}
let bike = Bike {
    name:"kawaski".into(),
    speed:250,
    model_name:"ninja".into()
};
let bike2 = bike.clone();
println!("{:?}",bike);
println!("{:?}",bike2);

}