fn main() {
    let peaches: i32 = {
        println!("I'm about to figure out how many peaches there are");
        let x = 10 + 5;
        println!("Now I know how many peaches there are");
        x
    };
    println!("I have {} peaches", peaches);
}