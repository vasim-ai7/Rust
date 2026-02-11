// struct User{
//     active :bool,
//     username :String,
//     email : String,
//     sign_in_count : u64,
// }

// fn main(){
//     let vasim_as_user1=User{
//         active :true,
//         username : String::from("vasim"),
//         email : String:: from ("vasim@gmail.com"),
//         sign_in_count : 1,
//     };
//     println!("User 1 username {:?}",vasim_as_user1.username);
//     println!("User 1 EMAIL {:?}",vasim_as_user1.email);
//     println!("User 1 active {:?} and sign in count {:?} ",vasim_as_user1.active,vasim_as_user1.sign_in_count);
// }



/////////////////////////////////
struct Rect{
    length : u64,
    breadth : u64,
}
impl Rect{
    fn area(&self) ->u64{.   /// than here in the argument after self we can declare that and use it 
            self.length * self.breadth
    }
    //multiple functions can be implemented here


    // suppose i want to creat a new function here without self ye we can do that
    fn debug(){
        return 1;
    }
    // but we cannnot call this directly using the object we need to use struct name along with ::
    //just like a static function
}

fn main(){
    let recc=Rect{
        length : 4,
        breadth: 6,
    };
    println!("area is {} ",recc.area())// here if we want to pass the argument than we xan do that
    println!("area is {} ",Rect::debug())
}