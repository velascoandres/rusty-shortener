use sqlx::{postgres::PgPoolOptions, Pool, Postgres};


pub type ConnectionPool = Pool<Postgres>;

pub async fn get_pool() -> ConnectionPool{    
    PgPoolOptions::new()
        .connect("postgresql://dev_user:dev_password@localhost:5432/pocket_link_postgres_dev")
        .await
        .expect("Error building connection pool")
}
