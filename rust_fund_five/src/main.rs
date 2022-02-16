fn main() {
    let float_thirty_two: f32 = 17.2;
    let unsigned_eight:u8 =5;
    let cast_unsigned_eight = unsigned_eight as f32;

    let result = float_thirty_two / cast_unsigned_eight;
    println!("{}", result);
}
