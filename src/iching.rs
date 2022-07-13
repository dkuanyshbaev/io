// ---------------------------------------
// IOracle iching
// ---------------------------------------
use futures::TryFutureExt;
use rocket_db_pools::{sqlx, sqlx::Row, Connection, Database};

pub type Hexagram = String;

pub struct Answer {
    pub question: String,
    pub answer: String,
    pub hexagram: Hexagram,
    pub r_hexagram: Hexagram,
}

impl Answer {
    pub fn new(question: String, hexagram: Hexagram, r_hexagram: Hexagram) -> Self {
        // ---------------------------------------
        // TODO: generate answer to question!
        // ---------------------------------------
        let answer = "42".to_string();
        // ---------------------------------------

        Answer {
            question,
            answer,
            hexagram,
            r_hexagram,
        }
    }
    pub fn get_by_id(mut db: Connection<crate::Db>, id: u32) -> Self {
        // ---------------------------------------
        // TODO: db search, return answer
        // ---------------------------------------
        // sqlx::query("SELECT content FROM logs WHERE id = ?")
        //     .bind(id)
        //     .fetch_one(&mut *db)
        //     .map_ok(|r| Db(r.content))
        //     .await;
        //
        // sqlx::query("SELECT content FROM logs WHERE id = ?")
        //     .bind(id)
        //     .fetch_one(&mut *db)
        //     .await
        //     .and_then(|r| Ok(r.try_get(0)?))
        //     .ok();

        Answer {
            question: "question".to_string(),
            answer: "42".to_string(),
            hexagram: "111000".to_string(),
            r_hexagram: "000111".to_string(),
        }
    }
    pub fn save(self, mut db: Connection<crate::Db>) -> u64 {
        // ---------------------------------------
        // TODO: save to db, return id
        // ---------------------------------------

        42
    }
}
