use std::io::stdin;

fn io_main() {
    let args = std::env::args();
    println!("{:?}", args);
    for arg in args {
        println!("{}", arg);
    }

    let mut str_buf = String::new();

    stdin().read_line(&mut str_buf).expect("Failed to read line.");

    println!("Your input line is \n{}", str_buf);
}