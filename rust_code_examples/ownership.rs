

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn printer(value: String) -> String {
    return String::from("3")
}

fn main() {
    let a = 1;
    let b = 2;
    println!("{} + {} = {}", a, b, add(a, b));


    let x = String::from("Hello");
    println!(" The result of printer is {}", printer(x));
    // println!(" {} shouldn't be accessible ", (x));


    println!("a is {}", a);
}

