use super::config;
use sqlx::{postgres::PgPoolOptions, PgPool};

trait PgConnection {
    fn get_pool(&self, max: u32) -> PgPool;
}

struct Database;

impl PgConnection for Database {
    fn get_pool(&self, max: u32) -> PgPool {
        todo!();
    }
}
