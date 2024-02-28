use rusqlite::{Connection, Result};

// Struct for the commands table
// #[allow(dead_code)] is used to suppress the warning for the struct being unused
// #[derive(Debug)] is used to derive the Debug trait for the struct
#[allow(dead_code)]
#[derive(Debug)]
struct Commands {
    id: i32,
    command: String,
    description: Option<String>,
}


fn new_table(conn: &Connection) -> Result<()> {
    // Create a new table
    conn.execute(
        "CREATE TABLE commands (
            id    INTEGER PRIMARY KEY,
            command  TEXT NOT NULL,
            description  TEXT
        )",
        (), // empty list of parameters.
    )?;

    // Create a new command struct
    let me = Commands {
        id: 0,
        command: "docker compose up -d".to_string(),
        description: Some("Starts the docker containers in the background".to_string()),
    };

    // Insert the command
    conn.execute(
        "INSERT INTO commands (command, description) VALUES (?1, ?2)",
        (&me.command, &me.description),
    )?;

    
    Ok(())
}

fn get_commands(conn: Connection) -> Result<()> {
    // Get the commands from the table
    let mut stmt = conn.prepare("SELECT * FROM commands")?;
    let command_iter = stmt.query_map([], |row| {
        Ok(Commands {
            id: row.get(0)?,
            command: row.get(1)?,
            description: row.get(2)?,
        })
    })?;

    // Print the commands
    for command in command_iter {
        println!("Found command: {:?}", command?);
    }
    Ok(())
}

pub fn main() -> Result<()> {
    // Create a new in-memory database
    let conn = Connection::open_in_memory()?;

    let _ = new_table(&conn); // Ignore the Result
    let _ = get_commands(conn); // Ignore the Result
    Ok(())
}

// Tests
#[cfg(test)]
mod rusqlite_tests {
    use super::*;

    #[test]
    fn test_new_table() {
        assert_eq!(new_table(&Connection::open_in_memory().unwrap()), Ok(()));
    }

    #[test]
    fn test_get_commands() {
        // Only set to not equal because no data is set here
        assert_ne!(get_commands(Connection::open_in_memory().unwrap()), Ok(()));
    }
}