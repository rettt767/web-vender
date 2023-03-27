use std::fs::read_to_string;
use schemars::schema::RootSchema;
use serde_json::to_string_pretty;
use crate::yml_loader::models::{EnvConfig, GlobalConfig};
use serde_yaml::from_str as yml_from;
use serde_json::from_str as to_json;

fn load_env_config() -> Option<EnvConfig>{
    let schema = yml_from::<RootSchema>(
        &read_to_string("resources/application.yml")
            .expect("Fail to found application.yml"),
    );
    return match schema {
        Ok(json) => {
            let data = to_string_pretty(&json)
                .expect("The file application.yml has syntax error");
            let p = to_json(&*data)
                .expect("Fail to transport message to JSON");
            return Some(p)
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

fn load_global_config(action: String) -> Option<GlobalConfig>{

}

pub fn load_config() -> Option<GlobalConfig>{

}