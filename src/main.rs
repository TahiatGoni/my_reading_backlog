use rusqlite::{Connection, Result};
use std::env;
mod storage;

fn print_help() {
    print!(
        "usage:
            --add <link>
        "
    )
}

fn main() -> Result<()> {
    let mut conn = Connection::open("reading_items.db")?;

    match storage::create_tables(&conn) {
        Ok(_res) => {}
        Err(err) => eprintln!("{}", err),
    };

    let args: Vec<String> = env::args().collect();
    //only implemented arg --add for now
    if args[1] == "--add" {
        match storage::add_item(&mut conn, &args[2]) {
            Ok(_res) => println!("ok"),
            Err(err) => eprintln!("{}", err),
        };
    } else {
        print_help();
    }
    Ok(())
}
