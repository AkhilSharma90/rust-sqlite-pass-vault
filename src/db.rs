use std::io;
use std::io::Write;
extern crate rusqlite;
use rusqlite::{Connection, Error};
use serde::{Deserialize, Serialize};


// This code defines the a struct ServiceInfo 
// for storing all the data of what is to be stored in the database.
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub id: Option<i64>,
    pub service: String,
    pub username: String,
    pub password: String,
}

// Constructor to initialize new struct ServiceInfo
impl ServiceInfo {
    pub fn new(service: String, username: String, password: String) -> Self {
        ServiceInfo {
            id: None,
            service,
            username,
            password,
        }
    }
}

// Prompt function takes in a string as argument 
// and Displays it while also taking in one input and returns the input
pub fn prompt(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

// function to initialize the database
//      Creates the database if it does not exist 
//      Creates a table 'passwords' if it does not exist with column matching with the Struct ServiceInfo
pub fn init_database() -> Result<Connection, Error> {
    let conn = Connection::open("passwords.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS passwords (
            id INTEGER PRIMARY KEY,
            service TEXT,
            username TEXT,
            password TEXT
        )",
        [],
    )?;

    Ok(conn)
}

// This function is to write the Data to the password table
// Takes 2 main input as parameters
    // 1. Connection variable to the database
    // 2. All the parameters to put inside database
// Then inserts them into the database with INSERT command and the input as parameters
    pub fn write_password_to_db(
    conn: &Connection,
    service: &str,
    username: &str,
    password: &str,
) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO passwords (service, username, password) VALUES (?, ?, ?)",
        &[&service, &username, &password],
    )?;
    Ok(())
}

// This function reads all the data from the passwords table and returns all the ENTRIES from the table.
pub fn read_passwords_from_db(conn: &Connection) -> Result<Vec<ServiceInfo>, Error> {
    let mut stmt = conn.prepare("SELECT service, username, password FROM passwords")?;
    let entries = stmt
        .query_map([], |row| {
            Ok(ServiceInfo::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(entries)
}

// This function searches for a service entry by name in the database
    // Statement to connect to database
    // Result is used to query a row from the passwords table and pass in the service name as parameters
    // Return the row after checking errors and no query returns  
pub fn search_service_by_name(conn: &Connection, name: &str) -> Result<Option<ServiceInfo>, Error> {
    let mut stmt = conn.prepare("SELECT id, service, username, password FROM passwords WHERE service = ?")?;
    let result = stmt.query_row(&[name], |row| {
        Ok(ServiceInfo {
            id: Some(row.get(0)?),
            service: row.get(1)?,
            username: row.get(2)?,
            password: row.get(3)?,
        })
    });

    match result {
        Ok(entry) => Ok(Some(entry)),
        Err(Error::QueryReturnedNoRows) => Ok(None),
        Err(err) => Err(err),
    }
}