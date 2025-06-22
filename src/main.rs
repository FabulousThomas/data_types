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

fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}

fn slices() {
    let slice = String::from("1, 2, 3, 4, 5");
    println!("slice = {}", slice);
    let sliced = &slice[3..7];
    println!("slice[2 & 3] = {}", sliced);
}

fn structs() {
    #[derive(Debug)]
    struct Users {
        username: String,
        email: String,
        password: String,
    }

    let user = Users {
        username: String::from("john_doe"),
        email: String::from("johndoe@email.com"),
        password: String::from("password123"),
    };

    println!("User = {:#?}", user);
    // println!("User = {}", user.email);
    // println!("User = {}", user.password);
}

fn implementations() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Rectangle width: {}, height: {}", rect.width, rect.height);
    println!("Area of Rectangle: {}, ", rect.area());
}

fn enums() {
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl Direction {
        fn is_vertical(&self) -> &str {
            match self {
                Direction::Up | Direction::Down => "YES",
                _ => "NO",
            }
        }
    }
    let direction = Direction::Left;
    match direction {
        Direction::Up => println!("Moving Up"),
        Direction::Down => println!("Moving Down"),
        Direction::Left => println!("Moving Left"),
        Direction::Right => println!("Moving Right"),
    }
    println!("Is direction vertical? {}", direction.is_vertical());
}

fn matchs() {
    let number = 1;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Not One, Two, or Three"),
    }
}

fn main() {
    self::integers();
    self::floats();
    self::tuples();
    self::arrays();

    let result = self::sum(1, 3);
    println!("Result of sum: {}", result);

    self::slices();
    self::structs();
    self::implementations();
    self::enums();
    self::matchs();
    // Uncomment the following line to see the panic
    // self::strings();
    // self::vectors();
    // self::hashmaps();
}
