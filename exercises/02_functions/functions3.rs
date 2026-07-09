fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    call_me(5);
    println!("\nIn this exercise,I have corrected the function call by providing an argument for the function being called.")
}
