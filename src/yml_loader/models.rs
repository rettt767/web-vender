use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GlobalConfig{
    pub app: AppConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Profile{
    pub active: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EnvConfig{
    pub profile: Profile,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MySQLConfig{
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppConfig{
    pub name: String,
    pub data_source_config: DataSourceConfig,
    pub server_config: ServerConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DataSourceConfig{
    pub mysql: MySQLConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerConfig{
    pub port: u32,
    pub hostname: String,
}