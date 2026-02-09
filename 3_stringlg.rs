fn get_string_length(s :&str)-> usize {
    s.chars().count()
    // no. return no semicolon then implicit return 
}
fn main (){
    let v= String::from("hello world");
    let my_str_length= get_string_length(&v);
    println!("The no of characters in my string is :{}",my_str_length);
}