/*An actix Microservice for Simple Calculator that has multiple routes:
A. / that turns a hello world message
B. /calculate/{string} that calculates the result of the string
*/

// use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
// use serde::{Serialize};

// //create a function that returns hello world
// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello World! Translators are fun!")
// }

// //create a function that returns the result of the translation
// #[get("/dialogue/{input}")]
// async fn calculate(input: web::Path<String>) -> impl Responder {
//     println!("input: {}", input);
//     let result = translate::translate_text(input.to_string());
//     HttpResponse::Ok().body(result.to_string())
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(hello).service(calculate))
//         .bind("0.0.0.0:8080")?
//         .run()
//         .await
// }

use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "yuzhou",
    about = "A Hugging Face Translation Tool in Rust"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "yuzhou")]
    Translate {
        #[clap(short, long)]
        text: String,
    }
}
// create main function that uses lib.rs
// fn main() -> anyhow::Result<()> {
//     let mut user_input = String::new();
//     let stdin = io::stdin("Ask your Question: "); // We get `Stdin` here.
//     stdin.read_line(&mut user_input);

//     println!("input {} ", user_input);
//         Some(Commands::Translate { text }) => {
//             println!("Response: {}", translate::translate_text(text));
//         }
//         None => {
//             println!("No command given");
//         }
//     }
//     Ok(())
// }

use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lock().lines();
    let mut user_input = Vec::<String>::new();

    println!("Enter a text you want to translate into Spanish. Press enter on an empty line to stop.");
    while let Some(line) = lines.next() {
        let last_input = line.unwrap();

        // stop reading
        if last_input.len() == 0 {
            break;
        }

        // store user input
        user_input.push(last_input);
    }

    translate::translate_text(user_input);

    // the lock is released after it goes out of scope
    Ok(())
}
