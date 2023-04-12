/* use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

pub fn create_db() -> Result<Connection> {
    let database_file = "data.db";
    let conn = Connection::open(database_file)?;
    //let _ = conn.exceute("DROP TABLE person", []);
    conn.execute(
        "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
        (), // empty list of parameters.
    )?;
    Ok(conn)
}

pub fn insert_db(conn: &Connection) -> Result<()> {
    let p1 = Person {
        id: 1,
        name: "Dave".to_string(),
        data: None,
    };
    let p2 = Person {
        id: 2,
        name: "Jack".to_string(),
        data: None,
    };
    conn.execute(
        "insert into person(id,name,data) values(?1,?2,?3),(?4,?5,?6);",
        params![p1.id, p1.name, p1.data, p2.id, p2.name, p2.data],
    )?;
    Ok(())
}

pub fn get_data(conn: &Connection){
    let mut stmt = conn.prepare("select id ,name ,data from person").unwrap();
    let persons_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    }).unwrap();
    let mut persons = Vec::new();
    for p in persons_iter {
        persons.push(p);
    }
    for person in persons{
        println!("{:?}",person);
     }
}
 */