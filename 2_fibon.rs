// fn main(){
//     let n=3;
//     println!("{}",res(n));
// }

// fn res(n : i32)-> i32{
//     if n <=1 {
//         return n;
//     }
//     else {
//         return res(n-1)+res(n-2);
//     }
// }

//another way

fn main(){
    let x: i32 =3;
    println!("{}",fib(x));
}


fn fib(num : i32)->i32{
    let mut first=0; // mut means the value can be changed
    let mut second=1;

    if num==0 {return first;}
    if num==1 {return second;}
    for mut i in  1..num -1{
        let temp =second;
        second =second+first;
        first=temp;
        i+=1;
    }
    return second;
}