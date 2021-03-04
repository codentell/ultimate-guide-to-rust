// Add in types for all of the variables in the next bit of code,
// and guess what the output of this program will be before running it. 
// I put an underscore (_) in front of some variable names because they are not used,
// and I don’t want the compiler to warn you about them. Feel free to remove those underscores if you like.

// fn main() {
//     let name = "Kody";
//     let x = println!("My name is {}", name);
//     let this_year = 2019;
//     let birth_year = 1985;
//     let age = this_year - birth_year;
//     let y = println!("I turned {} in {}", age, this_year);
//     assert_eq!(x, y);
//     let _z = println!("Thanks for chatting with me!");
// }

fn main() {
    let name: &str = "Kody";
    let x = println!("My name is {}", name);
    let this_year: i32 = 2020;
    let birth_year: i32 = 1985;
    let age: i32 = this_year - birth_year;
    let y = println!("I turned {} in {}", age, this_year);
    assert_eq!(x, y);
    let _z = println!("Thanks for chatting with me!");
}