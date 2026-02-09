fn main(){
    let ans=even_no(10);
    println!("{}",ans);
}

fn even_no (num :i32) -> bool{
if num % 2 == 0 {return true;}
else {return false;}

}

// i it can be signed  
// u it is unsigned no -> to store more data
//i64 also we can use for big no
