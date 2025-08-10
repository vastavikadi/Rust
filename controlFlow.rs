fn main() {
    let n = 5;

    if n > 0 {
        println!("Positive");
    } else if n < 0 {
        println!("Negative");
    } else {
        println!("Zero");
    }

    // Loop
    for i in 0..5 {
        println!("i = {}", i);
    }

    let mut count = 0;
    while count < 3 {
        println!("count = {}", count);
        count += 1;
    }

    let mut num = 0;
    loop {
        num += 1;
        if num == 3 {
            break;
        }
    }
}
