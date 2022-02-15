fn main() {
    // Tuples can hold mixed values, accessing them uses the syntax var_name.#
    let location:(&str, f64, f64) = ("KCLE", 41.4094069, -81.8546911);
    // Arrays can only hold a single data type, accessing them uses the syntax var_name[#]
    let some_numbers: [f32;3] = [5.02, 4.08, 3.20];
    println!("Location name: {}, latitude: {}, longitude: {}", location.0, location.1, location.2);
    println!("{}, {}, {}",some_numbers[0], some_numbers[1], some_numbers[2]);

    //you can also destructure tuples and arrays for easier syntax.
    let (name, latitude, longitude) = location;
    println!("Location name: {}, latitude: {}, longitude:{}", name, latitude, longitude);

}
