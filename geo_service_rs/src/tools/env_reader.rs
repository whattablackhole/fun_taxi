use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::sync::OnceLock;

pub struct AppEnv {
    env_map: HashMap<String, String>,
}

impl AppEnv {
    pub fn new(map: HashMap<String, String>) -> Self {
        return Self { env_map: map };
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.env_map.get(key)
    }
}

pub fn get_env() -> &'static AppEnv {
    static APP_ENV: OnceLock<AppEnv> = OnceLock::new();
    APP_ENV.get_or_init(|| AppEnv::new(init_env()))
}

fn init_env() -> HashMap<String, String> {
    let mut env_map: HashMap<String, String> = HashMap::new();

    if let Ok(result) = env::current_dir() {
        let mut env_path = result.to_str().unwrap().to_string();
        env_path.push_str("/.env");

        if let Ok(file) = File::open(env_path) {
            let buff_reader = std::io::BufReader::new(file);
            let line_itter = buff_reader.lines().map(|line| line.unwrap());

            line_itter.for_each(|l| {
                if let (Some(var_name), Some(var_value)) = l.split_once('=').unzip() {
                    env_map.insert(var_name.to_string(), var_value.to_string());
                }
            });
        }
    }

    env_map
}
