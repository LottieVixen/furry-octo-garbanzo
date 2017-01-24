fn main() {
    let result = is_greater(30,29);
    println!("{}",result);
}

fn is_greater(x:i32,y:i32) -> bool {
    if x > y {
       true
    } else {
       false
    }
}
