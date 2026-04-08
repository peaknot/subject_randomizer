use rusqlite::Connection;

use crate::{
    errors::AppError,
    structs::{Module, Subject, Topic, module::Randomized},
};

pub fn rand(connection: Connection) -> Result<Randomized, AppError> {
    let mut stmt = connection.prepare(
        "
        WITH random_module AS (
            SELECT m.id, m.mod_name
            FROM module m
            WHERE EXISTS (
                SELECT 1
                FROM subject s
                JOIN topic t ON t.subj_id = s.id
                WHERE s.mod_id = m.id
            )
            ORDER BY RANDOM()
            LIMIT 1
        ),
        random_subject AS (
            SELECT s.id, s.subj_name, s.mod_id
            FROM subject s
            WHERE s.mod_id = (SELECT id FROM random_module)
              AND EXISTS (
                  SELECT 1
                  FROM topic t
                  WHERE t.subj_id = s.id
              )
            ORDER BY RANDOM()
            LIMIT 1
        ),
        random_topic AS (
            SELECT t.id, t.topic_name, t.subj_id
            FROM topic t
            WHERE t.subj_id = (SELECT id FROM random_subject)
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
        JOIN random_subject rs ON rs.mod_id = rm.id
        JOIN random_topic rt ON rt.subj_id = rs.id;
    ",
    )?;

    let result: Randomized = stmt.query_row([], |row| {
        Ok(Randomized {
            module_id: row.get(0)?,
            module_name: row.get(1)?,
            subject_id: row.get(2)?,
            subj_name: row.get(3)?,
            topic_id: row.get(4)?,
            topic_name: row.get(5)?,
        })
    })?;

    Ok(result)
}

pub fn rand_module(connection: Connection) -> Result<Module, AppError> {
    let mut stmt = connection.prepare(
        "
            SELECT id, mod_name FROM module
            ORDER BY RANDOM()
            LIMIT 1
        ",
    )?;
    let result: Module = stmt.query_row([], |row| {
        Ok(Module {
            id: row.get(0)?,
            mod_name: row.get(1)?,
        })
    })?;

    Ok(result)
}

pub fn rand_subject(connection: Connection) -> Result<Subject, AppError> {
    let mut stmt = connection.prepare(
        "
            SELECT * FROM subject
            ORDER BY RANDOM()
            LIMIT 1;
        ",
    )?;
    let result: Subject = stmt.query_row([], |row| {
        Ok(Subject {
            id: row.get(0)?,
            subj_name: row.get(1)?,
            mod_id: row.get(2)?,
        })
    })?;

    Ok(result)
}

pub fn rand_topic(connection: Connection) -> Result<Topic, AppError> {
    let mut stmt = connection.prepare(
        "
            SELECT * FROM topic
            ORDER BY RANDOM()
            LIMIT 1;
        ",
    )?;
    let result: Topic = stmt.query_row([], |row| {
        Ok(Topic {
            id: row.get(0)?,
            topic_name: row.get(1)?,
            subj_id: row.get(2)?,
        })
    })?;

    Ok(result)
}
