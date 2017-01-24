fn main() {
    let mut x = 17;
    print_val(x);
    x = my_func();
    print_val(x);
    x = calc(x, 10);
    print_val(x);
}

fn my_func() -> i32 {
    let x = 9;
    x
}

fn calc(x:i32,y:i32) -> i32 {
    let result = x * y;
    result
}

fn print_val(val:i32) {
    println!("The value is {}",val);
}
