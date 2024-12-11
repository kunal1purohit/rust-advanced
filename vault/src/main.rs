mod pentry;
use crate::pentry::prompt;
use crate::pentry::read_passwords_from_file;
use crate::pentry::ServiceInfo;
use serde::{Deserialize, Serialize};

fn clr() {
    print!("{}[2J", 27 as char);
}

fn main() {
    clr();
    let ascii = r#"
                                                            .___                     .__   __   
___________    ______ ________  _  _____________  __| _/ ___  _______   __ __|  |_/  |_ 
\____ \__  \  /  ___//  ___/\ \/ \/ /  _ \_  __ \/ __ |  \  \/ /\__  \ |  |  \  |\   __\
|  |_> > __ \_\___ \ \___ \  \     (  <_> )  | \/ /_/ |   \   /  / __ \|  |  /  |_|  |  
|   __(____  /____  >____  >  \/\_/ \____/|__|  \____ |    \_/  (____  /____/|____/__|  
|__|       \/     \/     \/                          \/              \/                 
    "#;

    println!("{ascii}");
    loop {
        println!("password manager menu:");
        println!("1. add entry");
        println!("2. list entries");
        println!("3. search entry");
        println!("4. quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("service : "),
                    prompt("username : "),
                    prompt("password : "),
                );
                println!("entry added succesfully.");
                entry.write_to_file();
            }
            "2" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("error reading passwords:{}", err);
                    Vec::new()
                });
                for item in &services {
                    println!(
                        "Services = {}
                        -Username : {}
                        -Password : {}",
                        item.service, item.username, item.password
                    );
                }
            }
            "3" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("error reading passwords:{}", err);
                    Vec::new()
                });
                let search = prompt("Search :");
                for item in &services {
                    if item.service.as_str() == search.as_str() {
                        println!(
                            "Service = {}
                            -Username : {}
                            -Password : {}",
                            item.service, item.username, item.password
                        );
                    }
                }
            }
            "4" => {
                clr();
                println!("Goodbye!");
                break;
            }
            _ => println!("invalid choice."),
        }
        println!("\n\n");
    }
}
