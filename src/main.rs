use std::io;

fn main() {

    println!("gæt et tal");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("du er dum knep dig selv");

    println!("your number is {guess}");
    unsafe {
        println!("The actual number is {}",randmy());
    }
}

    extern "C" {
        fn randmy () -> i32;
    }


