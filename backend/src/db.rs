use diesel::mysql::MysqlConnection; // Changed from PgConnection
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>; // Changed from PgConnection

// Function to get a connection from the pool (used in handlers)
// Corrected the return type to reflect MysqlConnection
pub fn establish_connection(pool: &DbPool) -> Result<r2d2::PooledConnection<ConnectionManager<MysqlConnection>>, actix_web::Error> {
    pool.get().map_err(|e| {
        log::error!("Failed to get DB connection: {:?}", e);
        // Corrected the truncated error message string
        actix_web::error::ErrorInternalServerError("Failed to connect to database")
    })
}
