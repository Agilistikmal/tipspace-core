use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

pub trait Database {
    fn get_connection_pool(&self) -> Pool<ConnectionManager<PgConnection>>;
}

pub struct PgDatabase {
    url: String,
}

impl PgDatabase {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

impl Database for PgDatabase {
    fn get_connection_pool(&self) -> Pool<ConnectionManager<PgConnection>> {
        let manager = ConnectionManager::<PgConnection>::new(&self.url);

        Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Failed to build connection pool")
    }
}
