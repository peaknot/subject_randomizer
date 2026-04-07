use rusqlite::Connection;

use crate::{errors::AppError, structs::module::Randomized};


pub fn rand(connection: Connection) -> Result<Randomized, AppError>  {
    
    let mut stmt = connection.prepare("
        WITH random_module AS (
            SELECT id, mod_name
            FROM module
            ORDER BY RANDOM()
            LIMIT 1
        ),
        random_subject AS (
            SELECT id, subj_name, mod_id
            FROM subject
            WHERE mod_id = (SELECT id FROM random_module)
            ORDER BY RANDOM()
            LIMIT 1
        ),
        random_topic AS (
            SELECT id, topic_name, subj_id
            FROM topic
            WHERE subj_id = (SELECT id FROM random_subject)
            ORDER BY RANDOM()
            LIMIT 1
        )
        SELECT 
            rm.id   AS module_id,
            rm.mod_name AS module_name,
            rs.id   AS subject_id,
            rs.subj_name,
            rt.id   AS topic_id,
            rt.topic_name
        FROM random_module rm
        JOIN random_subject rs
        JOIN random_topic rt;
    ")?;

    let result = stmt.query_row(
[], 
        |row| {
            Ok(Randomized {
                module_id:  row.get(0)?,
                module_name: row.get(1)?,
                subject_id: row.get(2)?,
                subj_name: row.get(3)?,
                topic_id: row.get(4)?,
                topic_name: row.get(5)?
            })
        }
    )?;
    
    Ok(result)
}