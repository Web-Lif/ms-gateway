use sqlx::{ Pool, MySql };
pub struct AppGlobalData{
    pub pool: Pool<MySql>
}