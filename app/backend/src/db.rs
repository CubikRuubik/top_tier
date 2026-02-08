use rusqlite::{Connection, Result};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS entries (
                title TEXT NOT NULL UNIQUE,
                pubkey TEXT PRIMARY KEY
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    pub fn search(&self, query: &str) -> Result<Vec<(String, String)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT title, pubkey FROM entries WHERE title LIKE ?1")?;

        let pattern = format!("%{}%", query);
        let rows = stmt.query_map([pattern], |row| Ok((row.get(0)?, row.get(1)?)))?;

        rows.collect()
    }

    pub fn insert(&self, title: &str, pubkey: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO entries (title, pubkey) VALUES (?1, ?2)",
            [title, pubkey],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_db() -> Database {
        Database::new(":memory:").expect("Failed to create test database")
    }

    #[test]
    fn test_insert_and_search() {
        let db = setup_test_db();

        db.insert("My Test Entry", "ABC123").unwrap();

        let results = db.search("Test").unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, "My Test Entry");
        assert_eq!(results[0].1, "ABC123");
    }

    #[test]
    fn test_search_no_results() {
        let db = setup_test_db();

        db.insert("Something", "ABC123").unwrap();

        let results = db.search("Nothing").unwrap();

        assert!(results.is_empty());
    }

    #[test]
    fn test_search_partial_match() {
        let db = setup_test_db();

        db.insert("Hello World", "AAA").unwrap();
        db.insert("Hello There", "BBB").unwrap();
        db.insert("Goodbye", "CCC").unwrap();

        let results = db.search("Hello").unwrap();

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_duplicate_pubkey_fails() {
        let db = setup_test_db();

        db.insert("First", "ABC123").unwrap();
        let result = db.insert("Second", "ABC123");

        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_title_fails() {
        let db = setup_test_db();

        db.insert("Same Title", "AAA").unwrap();
        let result = db.insert("Same Title", "BBB");

        assert!(result.is_err());
    }
}
