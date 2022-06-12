fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.
    // Constants aren’t just immutable by default—they’re always immutable.
    // Constants are valid for the entire time a program runs, within the scope they were declared in.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!(
        "The value of three hours in seconds is: {}",
        THREE_HOURS_IN_SECONDS
    );
}
