// Write a program that prints the result of doing 5 + 3, then 6 - 2, 
// then multiplying the two results together.
// 1. Do it once using multiple variables.
// 2. Do it once using a single variable.
// 3. Do it once using no variables at all.

fn main() {
    // Version 1 
    let x = 5 + 3;
    let y = 6 - 2;
    println!("5 + 3 * 6 - 2 = {}", x * y);

    // Version 2
    let x = (5 + 3) * (6 - 2);
    println!("5 + 3 * 6 - 2 = {}", x);

    // Version 3
    println!("5 + 3 * 6 - 2 = {}", (5 + 3) * (6 - 2));
}
