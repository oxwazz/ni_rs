use inquire::{InquireError, Select, Text};
use std::process::Command;

fn main() {
    let tes = Command::new("npm") // Can combine args into one string
        // .env("PATH", "/home/oxwazz/.nvm/versions/node/v20.17.0/bin:$PATH")
        .arg("-v")
        .output()
        .expect("Failed to execute npm command");
    let tes = String::from_utf8(tes.stdout).expect("errrrr");
    println!("{}", tes);
    let options: Vec<&str> = vec![
        "Banana",
        "Apple",
        "Strawberry",
        "Grapes",
        "Lemon",
        "Tangerine",
        "Watermelon",
        "Orange",
        "Pear",
        "Avocado",
        "Pineapple",
    ];

    let ans: Result<&str, InquireError> =
        Select::new("What's your favorite fruit?", options).prompt();

    match ans {
        Ok(choice) => println!("{}! That's mine too!", choice),
        Err(_) => println!("There was an error, please try again"),
    }
}
