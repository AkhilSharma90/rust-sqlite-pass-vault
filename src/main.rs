mod db;
use db::*;

// function named clr which clears the terminal screen
// It uses the [2J command which tells the terminal to clear the screen
fn clr() {
    print!("{}[2J", 27 as char);
}

// This are first few lines of the code, 
// it imports the db.rs file,
fn main() {
    // Making connection to the database
    let conn = init_database().expect("Failed to initialize the database");
    clr();
    // Here, the main function begins, first thing, it begins the connection to the database,
    // then it defines the asciii art and prints it.
    let ascii = r#"

    ________  ________  ________   ________           ___      ___ ________  ___  ___  ___   _________   
    |\   __  \|\   __  \|\   ____\ |\   ____\         |\  \    /  /|\   __  \|\  \|\  \|\  \ |\___   ___\ 
    \ \  \|\  \ \  \|\  \ \  \___|_\ \  \___|_        \ \  \  /  / | \  \|\  \ \  \\\  \ \  \\|___ \  \_| 
    \ \   ____\ \   __  \ \_____  \\ \_____  \        \ \  \/  / / \ \   __  \ \  \\\  \ \  \    \ \  \  
     \ \  \___|\ \  \ \  \|____|\  \\|____|\  \        \ \    / /   \ \  \ \  \ \  \\\  \ \  \____\ \  \ 
      \ \__\    \ \__\ \__\____\_\  \ ____\_\  \        \ \__/ /     \ \__\ \__\ \_______\ \_______\ \__\
       \|__|     \|__|\|__|\_________\\_________\        \|__|/       \|__|\|__|\|_______|\|_______|\|__|
                          \|_________\|_________|                                                        



    "#;
    println!("{ascii}");

    // This is the main loop
    loop {
        // This prints the Menu of the app
        // takes input from user in a variable named choice to then decide which action to do.
        println!("Password Manager Menu:");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Search");
        println!("4. Quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        // Here we use match to work according to the input
        // It clears the terminal first and then,
            // If the input is “1”, we get the input from user and write it to the database
        match choice.trim() {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service :"),
                    prompt("Username :"),
                    prompt("Password :"),
                );
                write_password_to_db(
                    &conn,
                    &entry.service,
                    &entry.username,
                    &entry.password,
                )
                .expect("Failed to write to the database");
                println!("Entry added successfully.");
    
            }
            // If choice is 2, clears the terminal first and then
                // This lets the user to list the stored service entries
                // And display all of them on the terminal using a for loop
            "2" => {
                clr();
                let services = read_passwords_from_db(&conn).unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);    
                    Vec::new()
                });
                for item in &services {
                    println!(
                        "Service = {}
    - Username : {} 
    - Password : {}",
                        item.service, item.username, item.password
                    );
                }
            }
            // This function lets the user search for the password and username by its service name
                // 1. Clear the terminal
                // 2. Uses a match on the function search_service_by_name to handle the various returns it can give
                // 3. If the query is found it prints the data, if it is not found it prints service not found, else prints the error thrown.
            "3" =>{
                clr();
                let search = prompt("Search by service name:");
                match search_service_by_name(&conn, &search) {
                    Ok(Some(entry)) => {
                        println!(
                            "Service = {}
                - Username : {} 
                - Password : {:?}",
                            entry.service, entry.username, entry.password
                        );
                    }
                    Ok(None) => {
                        println!("Service not found.");
                    }
                    Err(err) => {
                        eprintln!("Error searching for service: {}", err);
                    }
                }
            }

            // If the choice is “4”, it prints “Goodbye” and breaks out of the loop
            // And handles the error if the choice is not valid
            "4" => {
                clr();
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice."), // Handles all the inputs other than 1, 2, 3, 4 
        }
        println!("\n\n");
    }
}