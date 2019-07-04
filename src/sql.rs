extern crate postgres;

use postgres::{Connection, TlsMode};
use crate::data::Person;


pub fn establish_connection() -> Connection
{
    Connection::connect("postgres://postgres@localhost:5433", TlsMode::None).unwrap()
}

pub fn create_table(conn: &Connection)
{
    conn.execute("CREATE TABLE IF NOT EXISTS PERSON(id SERIAL PRIMARY KEY, name VARCHAR NOT NULL, age INTEGER)", &[]).unwrap();
}

pub fn insert(conn: &Connection, p: &Person)
{
    conn.execute("INSERT INTO PERSON(name, age) VALUES($1, $2)", &[&p.name, &p.age]).unwrap();
}

pub fn select(conn: &Connection, where_clause: &str) -> Vec<Person>
{
    let mut query = "SELECT * FROM PERSON".to_owned();
    query.push_str(where_clause);

    let stmt = conn.prepare(&query).unwrap();
    let mut result: Vec<Person> = vec![];
    for row in &stmt.query(&[]).unwrap()
        {
            let mut tmp = Person{id: row.get(0),name: row.get(1), age: row.get(2)};
            result.push(tmp);
        }
    return result;
}

pub fn delete(conn: &Connection, where_clause: &str)
{
    let mut query = "DELETE FROM PERSON".to_owned();
    query.push_str(where_clause);
    conn.execute(&query,&[]).unwrap();
}

pub fn update(conn: &Connection, update_clause: &str, where_clause: &str)
{
    let mut query = "UPDATE PERSON ".to_owned();
    query.push_str(update_clause);
    query.push_str(" ");
    query.push_str(where_clause);
    conn.execute( &query, &[]).unwrap();
}