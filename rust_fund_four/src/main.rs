fn main() {
    let duck = "Duck";
    let airline = "Airlines";
    //this is one way of combining string slices together.
    let airline_name = [duck, " ", airline].concat();
    println!("{}", airline_name);
    let airline_name_two = format!("{} {}", duck, airline);
    println!("{}", airline_name_two);

    let mut slogan = String::new();
    slogan.push_str("We hit the ground");
    slogan.push(' ');
    slogan = slogan + "every time";

    println!("{}", slogan)
}
