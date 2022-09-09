use std::{fs::File, io::Read, path::Path};

use once_cell::sync::Lazy;
use serde::Deserialize;

pub static CONFIG: Lazy<Config> =
    Lazy::new(|| Config::from_toml().expect("无法正确解析配置，请检查配置是否正确"));

#[derive(Debug, Deserialize)]
pub struct Config {
    pub debug: bool, // 调试开关
    #[serde(default = "default_env_flag")]
    pub env_flag: String, // 标识环境
    #[serde(default = "default_character")]
    pub character: String,
    pub http: HTTP,
    pub jwt: JWT,
    pub database: DATABASE,
}

#[derive(Debug, Deserialize)]
pub struct HTTP {
    pub host: [u8; 4], // 监听地址
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct JWT {
    #[serde(default = "default_jwt_secret")]
    pub secret: String,
    #[serde(default = "default_jwt_expire")]
    pub expire: u32, // access存活时间，毫秒
}

#[derive(Debug, Deserialize)]
pub struct DATABASE {
    pub url: String, // 数据库连接地址
    pub username: Option<String>,
    pub password: Option<String>,
}

fn default_env_flag() -> String {
    let _feature = "prov";

    #[cfg(feature = "local")]
    let _feature = "local";
    #[cfg(feature = "dev")]
    let _feature = "dev";
    _feature.into()
}

fn default_character() -> String {
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".into()
}

fn default_jwt_secret() -> String {
    "jwt_secret".into()
}

fn default_jwt_expire() -> u32 {
    120
}

impl Config {
    #[allow(unused)]
    fn set_env_from_file<T: AsRef<Path>>(path: T) -> anyhow::Result<()> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        for line in content.lines() {
            match line.find('=') {
                None => (),
                Some(index) => {
                    let key = &line[..index];
                    if std::env::var(key).is_err() {
                        std::env::set_var(key, &line[index + 1..]);
                    }
                }
            }
        }
        Ok(())
    }

    fn read_config() -> anyhow::Result<String> {
        let path = std::env::var("CONFIG").unwrap_or("./config.toml".into());
        let mut buf = String::new();
        let mut f = File::open(&path)?;
        f.read_to_string(&mut buf)?;
        Ok(buf)
    }

    pub fn from_toml() -> anyhow::Result<Self> {
        let config = Config::read_config()?;
        Ok(toml::from_str(&config)?)
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn test() {
        std::env::set_var("DEBUG", "false");
        match Config::from_toml() {
            Ok(config) => {
                assert!(!config.debug);
            }
            _ => {}
        }
    }
}
