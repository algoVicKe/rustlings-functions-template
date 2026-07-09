// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    num * num
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
    println!("In this exercise,I have removed the semicolon that was at the end of the function body in square() to enable automatic evaluation and hence implicit returns.")
}
