fn data_types() {
    let integer = 13;
    let float_and_double = 1.45;
    let boolean = true;
    let string = "String";
    let character = 'A';
}

// main function
fn main() {
    // every variable is immutable by default (we can't assign twice)
    // use mut to make a variable mutable (we can change the variable value)
    let mut mutable = "Dylan";
    mutable = "Bob";

    // macro function println! every macro function ends with a '!'
    println!("Hello");

    let mut result = add(1, 23);
    // result = 1;
    // result = -2;

    if result > 16 {
        println!("{}", result);
    } else if result == 1 {
        println!("{}", 1);
    } else {
        println!("{}", false);
    }

    // infinite loop
    let mut count = 0;

    loop {
        if count == 5 {
            break;
        }
        count += 1;
        println!("{}", count);
    }
    // classic while loop
    while count != 0 {
        count -= 1;
        println!("{}", count);
    }
}

fn basic_math_operations() {
    let addition = 1 + 2;
    let subtraction = 4 - 3;
    let multiplication = 10 * 2;
    let division = 34 / 2;
    let remainder = 4 % 2;
}

// parameters a and b are a 32-bit signed integer type
fn add(a: i32, b: i32) -> i32 {
    // returns a + b -> (a 32-bit signed integer) no need for semicolon! otherwise it's an error
    a + b
}
