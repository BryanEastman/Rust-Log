use chrono::prelude::*;
use std::env;
use rusqlite::{Connection, Result};
use std::io;
use std::error::Error;

#[derive(Debug)]
struct RoleDef {
    role: String,
    clock_state: bool,
    clock_time: String,
}

// fetch and add to user table
fn g() -> Result<(), Box<dyn Error>> {

    println!("get user table");

    let conn = Connection::open("log.db")?;

    let mut stmt = conn.prepare(
        "SELECT * 
        FROM roles"
    )?;
    
    let user_iter = stmt.query_map([], |row| {
        Ok(RoleDef {
            role: row.get(0)?,
            clock_state: row.get(1)?,
            clock_time: row.get(2)?
        })
    })?;

    for user in user_iter {
        println!("{:?}", user.unwrap());
    }
    Ok(())
}
fn a() -> Result<(), Box<dyn Error>> {
    println!("add user to roles table");
    let mut user = String::new();

    io::stdin()
        .read_line(&mut user)
        .expect("failed to read input");

    let user: String = user.trim().to_uppercase().to_string();
    println!("adding user: {}", user);
    let conn = Connection::open("log.db")?;

    conn.execute(
        "INSERT OR IGNORE INTO roles (role, clock_state, clock_time) 
        VALUES (?1, 0, DATETIME('now'))",
        &[&user]
    )?;

    println!("user: {} added", &user);

    Ok(())
}

// user log and clockin function
fn l() -> Result<(), Box<dyn Error>> {
    println!("log of user");

    let conn = Connection::open("log.db")?;
    
    Ok(())
}

fn clock() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("log.db")?;
    let mut user = String::new();

    println!("enter role to clock in/out");

    io::stdin()
        .read_line(&mut user)
        .expect("failed to read input");
    let user: String = user.trim().to_uppercase().to_string();

    conn.execute(
        "UPDATE roles
        SET 
            clock_state = CASE
                WHEN clock_state = 0 THEN 1
                ELSE 0
                END,
            clock_time = DATETIME('now')
        WHERE role = (?)",
        &[&user]
    )?;

    conn.execute(
        "INSERT INTO log
        SELECT * FROM roles
        WHERE role = (?)", &[&user]
    )?;

    println!("user {} successfully logged", user);
    Ok(())

}

// database initialization and deletion
fn d() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("log.db")?;

    println!("dropping tables log and roles");

    conn.execute(
        "DROP TABLE IF EXISTS log",
        (),
    )?;
    conn.execute(
        "DROP TABLE IF EXISTS roles",
        (),
    )?;

    Ok(())
}
fn b() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("log.db")?;

    println!("Creating users table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS roles (
            role TEXT PRIMARY KEY NOT NULL UNIQUE,            
            clock_state BOOLEAN NOT NULL,
            clock_time DATETIME NOT NULL
        )", ()
    )?;

    println!("Creating log table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS log (
            role TEXT NOT NULL,
            clock_state BOOLEAN NOT NULL,
            clock_time DATETIME NOT NULL
        )", ()
    )?;
    
    Ok(())
}

// help list of functions
fn h() -> Result<(), Box<dyn Error>> {
    println!("commands:");
    println!("g: gets a list of all roles and their clock states");
    println!("a: add a new role to roles table");

    println!("clock: clock in or out for role");
    println!("l: get full time log of a role");

    println!("b: initialize database");
    println!("d: delete all tables");
    //println!("c: clear target row from selected table");
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg1 = args[1].to_owned();
    let timestamp: DateTime<Local> = Local::now();

    match arg1.trim() {
        "g" => g(),
        "a" => a(),
        "l" => l(),
        "d" => d(),
        "b" => b(),
        "h" => h(),
        "clock" => clock(),
        // "c" => c(),
        _ => Ok(println!("invalid argument, run with option h to see a list of commands")),
    }.expect("error");
}
