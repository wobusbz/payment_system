mod dao;
use actix_web::{get, web, App, HttpServer, Responder};
use async_trait::async_trait;
use sqlx::mysql::MySqlPool;
use std::sync::Arc;
pub type Pool = MySqlPool;


#[get("/{id}/{name}/index.html")]
async fn index(
    state: actix_web::web::Data<dao::dao::Dao>,
    web::Path((id, name)): web::Path<(u32, String)>,
) -> impl Responder {
    format!("Hello {}! id:{} ", name, id )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let data = dao::dao::init_db_connection(std::env::var("DATABASE_URL").unwrap()).await;
    HttpServer::new(move || App::new().data(data.clone()).service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[derive(Debug, Clone)]
pub struct Db {
    pub database_url: String,
}

pub type SqlPool = sqlx::mysql::MySqlPool;
pub type AppStateRaw = std::sync::Arc<State>;
impl Db {
    pub fn new(url: String) -> Self {
        Db { database_url: url}
    }
    pub async fn init_db_connection(&self) -> AppStateRaw {
        let db_conn = sqlx::Pool::connect(&self.database_url).await.unwrap();
        Arc::new(State { sql: db_conn })
    }
}

#[derive(Clone)]
pub struct State {
    pub sql: SqlPool,
}

#[async_trait]
pub trait Wuhuarou {
    async fn user_add(&self) -> sqlx::Result<User>;
}

#[async_trait]
impl Wuhuarou for AppStateRaw {
    async fn user_add(&self) -> sqlx::Result<User> {
        sqlx::query_as!(User, r#"SELECT id, name FROM user WHERE id = ?"#, 1)
            .fetch_one(&self.sql)
            .await
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
}
