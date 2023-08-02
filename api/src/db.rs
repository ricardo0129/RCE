use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Executor};
use sqlx::postgres::PgPool;


#[derive(sqlx::FromRow)]
struct Question {
    qid: i32,
    name: String,
    problem_statement: String
}

#[derive(sqlx::FromRow)]
struct Testcase {
    input: String,
    output: String
}

struct DBHelper {
    pool: PgPool,
}

impl DBHelper {
    async fn new() -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://username:password!@localhost/codetable").await.unwrap();
        Self {pool: pool}
    }
    
    async fn get_tc(&self, qid: i32) -> Vec<Testcase> {
        let query = format!("SELECT input, output FROM (QUESTION CROSS JOIN TESTCASE) NATURAL JOIN HASTC WHERE qid={}", qid);
        let qs = sqlx::query_as::<Postgres, Testcase>(&query)
            .fetch_all(&self.pool).await.unwrap();
        qs
    }
    
    async fn get_qs(&self, qid: i32) -> Option<Question> {
        let query = format!("SELECT * FROM QUESTION WHERE qid={}", qid);
        let mut qs = sqlx::query_as::<Postgres, Question>(&query)
            .fetch_all(&self.pool).await.unwrap();
        match qs.len() {
            0 => None,
            _ => Some(qs.remove(0))
        }
    }

    async fn get_tc_qs(&self, qid: i32) -> Option<(Question,Vec<Testcase>)> {
        let q = self.get_qs(qid).await;
        if let Some(q) = q {
            return Some((q, self.get_tc(qid).await));
        }
        return None;
    }

}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db = DBHelper::new().await;
    db.get_tc(1).await;

    Ok(())
}
