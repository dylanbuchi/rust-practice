fn main() {
    infinite_loop();
    while_loop();
    for_loops();
}

fn infinite_loop() {
    let mut count = 0;
    println!("infinite loop");

    loop {
        if count == 5 {
            break;
        }
        count += 1;
        println!("{}", count);
    }
}
fn for_loops() {
    // for each item loop
    println!("For each item Loop");
    let a = [1, 2, 3, 4, 5];

    for item in a.iter() {
        println!("{}", item);
    }
    // for loop in a range
    println!("For Loop range exclusive end");
    // .. exclusive end
    for number in 1..3 {
        println!("{}", number)
    }

    // ..= inclusive end,
    println!("For Loop range inclusive end");
    for number in 1..=3 {
        println!("{}", number)
    }

    // for loop reverse items
    println!("For Loop in reverse");
    for number in (1..5).rev() {
        println!("{}", number)
    }
}

fn while_loop() {
    let mut count = 4;
    println!("While loop");

    while count != 0 {
        count -= 1;
        println!("{}", count);
    }
}
