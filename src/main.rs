fn integers() {
    let mut integer: i64 = 2147483647;
    integer = integer + 1;
    println!("integer = {}", integer);
}

fn floats() {
    let float: f64 = 1.0 / 3.0;
    println!("float = {}", float);
}

fn tuples() {
    let tuple: (i64, f32, &str) = (1, 2.0, "three");
    let (a, b, c) = tuple;
    println!("a = {}, b = {}, c = {}", a, b, c);
    println!("tuple = {:?}", tuple);
    let a = tuple.1;
    println!("a = {}", a);
}

fn arrays() {
    let array: [i64; 3] = [1, 2, 3];
    println!("array = {:?}", array);
    println!("Array[0] = {}", array[0]);
    // Uncommenting the next line will cause a panic
    // println!("array[3] = {}", array[3]);
}

fn main() {
    self::integers();
    self::floats();
    self::tuples();
    self::arrays();
    // Uncomment the following line to see the panic
    // self::slices();
    // self::strings();
    // self::structs();
    // self::enums();
    // self::vectors();
    // self::hashmaps();
}
