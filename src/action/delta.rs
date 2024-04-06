use chrono::{DateTime, Local};
use rusqlite::Connection;
use uuid::Uuid;

use super::{ActionError, ActionResult};

// the granular building block of an action. Each action can contain multiple deltas
enum Delta {
    Insert { position: usize, text: String },
    Delete { position: usize, length: usize },
}

struct Action {
    timestamp: DateTime<Local>,
    deltas: Vec<Delta>,
}

const CREATE_TABLE_QUERY: &str = "
    CREATE TABLE IF NOT EXISTS deltas (
        id INTEGER PRIMARY KEY,
        action_id INTEGER,
        position INTEGER,
        text TEXT,
        length INTEGER
    );
";

impl Action {
    fn insert_into_db(&self, conn: &Connection) -> ActionResult<()> {
        // let tx = conn.transaction().map_err(ActionError::DatabaseError)?;
        // let action_id = Uuid::new_v4();

        // for delta in &self.deltas {
        //     match delta {
        //         Delta::Insert { position, text } => {
        //             tx.execute(
        //                 "INSERT INTO deltas (action_id, position, text) VALUES (?, ?, ?)",
        //                 &[&action_id, &(*position as i64), &text],
        //             )?;
        //         }
        //         Delta::Delete { position, length } => {
        //             tx.execute(
        //                 "INSERT INTO deltas (action_id, position, length) VALUES (?, ?, ?)",
        //                 &[&action_id, &(*position as i64), &(*length as i64)],
        //             )?;
        //         }
        //     }
        // }
        // tx.commit().map_err(ActionError::DatabaseError)?;
        todo!();
    }
}
