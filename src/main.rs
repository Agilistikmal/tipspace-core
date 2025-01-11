use infrastructure::database::Database;

mod delivery;
mod infrastructure;
mod model;
mod repositories;
mod services;

fn main() {
    let config = infrastructure::config::Config::load();
    let database = infrastructure::database::PgDatabase::new(config.database_url.clone());

    let pool = database.get_connection_pool();

    println!(
        "Pool: {} | {}",
        pool.state().connections,
        pool.state().idle_connections
    )
}
