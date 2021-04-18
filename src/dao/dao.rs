use std::sync::Arc;

pub type SqlPool = sqlx::mysql::MySqlPool;
pub type Dao = std::sync::Arc<DbDao>;

#[derive(Debug, Clone)]
pub struct DbDao  {
    db: SqlPool
}

pub async fn init_db_connection(uri : String) -> Dao {
    let db_conn = SqlPool::connect(&uri).await.unwrap();
    Arc::new(DbDao { db: db_conn })
}