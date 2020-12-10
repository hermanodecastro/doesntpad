extern crate rusqlite;

use rusqlite::{Connection, Error, ToSql};
use std::iter::IntoIterator;
use crate::models::Notepad;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Database {
    connection: Connection
}

impl Database {
    pub fn new() -> Result<Self> {
        Ok(Self {
            connection: Connection::open("database/doesntpad.db")?
        })
    } 

    pub fn select<P>(&mut self, query: &'static str, params: P) -> Result<Notepad>
    where
        P: IntoIterator,
        P::Item: ToSql
    {
        let mut stmt = self.connection.prepare(query)?;
        let result = stmt.query_row(params, |row| {
            Ok(Notepad {
                id: row.get(0)?,
                slug: row.get(1)?,
                content: row.get(2)?,
            })
        })?;
        Ok(result)
    }

    pub fn execute<P>(&mut self, query: &'static str, params: P) -> Result<()> 
    where 
        P: IntoIterator,
        P::Item: ToSql
    {
        self.connection.execute(query, params)?;
        Ok(())
    }

    pub fn exists<P>(&mut self, query: &'static str, params: P) -> Result<bool> 
    where 
        P: IntoIterator,
        P::Item: ToSql
    {
        let mut stmt = self.connection.prepare(query)?;
        Ok(stmt.exists(params)?)
    }

    pub fn close(self) -> Result<(), (Connection, Error)> {
        self.connection.close()?;
        Ok(())
    } 
}
