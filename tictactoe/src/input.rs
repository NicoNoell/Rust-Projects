use std::io;

pub fn get_index() -> usize {
    loop {
        let mut input = String::default();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Got an Error while reading from console!\n");
            continue;
        }

        let n = match input.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                println!("Please input a valid Number\n");
                continue;
            },
        };

        if n < 1 || n > 9 {
            println!("Number has to be between 1 and 9 (ends inclusive)\n");
            continue;
        }

        return (n-1) as usize;
    }
}