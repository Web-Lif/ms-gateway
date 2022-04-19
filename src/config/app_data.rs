use sqlx::{ Pool, MySql };
use super::application::Config;

pub struct AppGlobalData{
    pub pool: Pool<MySql>,
    pub config: Config
}