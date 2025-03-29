use std::{collections::HashMap, env, fs::File, io::Read};
use yaml_rust2::{Yaml, YamlLoader};

enum Services {
    AWS(AWS),
    GCP(GCP)
}

pub struct GCP {
    pub bucket: String
}

pub struct SplitFiles {
    pub enable: bool,
    pub amount_parts: u8,
}

pub struct Watch {
    folders: Vec<String>,
    extensions_supported: Vec<String>
}

pub struct Runtime {
    split_files: SplitFiles,
    watch : Watch
}

pub struct LocalDB {
    file_name: String
}
pub struct AWS {
    pub bucket: String
}

pub(crate) struct Config {
    storages_provider: HashMap<String, Services>
}
impl Config {
    pub fn get_config() -> Config {
        let doc = load_file_config();

        let keys = doc["storages_provider"].as_hash().unwrap();

        let services:HashMap<String, Services> = keys.iter().fold( HashMap::new(),  |mut acc, (key, value) | {
            let key = key.as_str().unwrap().to_ascii_lowercase();
            let services;
            if key == "aws" {
                services = Services::AWS(AWS{bucket: value["bucket"].as_str().unwrap().to_ascii_lowercase()});
            } else {
                services = Services::GCP(GCP{bucket: value["bucket"].as_str().unwrap().to_ascii_lowercase()});
            }
            acc.insert(key, services);

            return acc;
        });

        Config{
            storages_provider: services
        }
    }
    pub fn get_aws_config(&self) -> Option<&AWS> {
        let aws = self.storages_provider.get("aws").unwrap();
         return if let Services::AWS(aws_config) = aws { Some(aws_config) } else { None };
    }
}

/**
runtime:
  split_files:
    enable: false
    amount_parts: 0
  watch:
    folder: ./
    extensions_supported: ".jpg,.mp4"
**/

impl Runtime {

    fn get_folder(runtime_keys: &Yaml)-> Vec<String>{

        vec![]
    }
    pub fn get_config() -> Runtime {
        let doc = load_file_config();
        let runtime_keys = &doc["runtime"];
        let folders =runtime_keys["watch"]["folders"].as_str().unwrap().to_string().split(",")
            .map(|file|{
                file.to_string()
            }).collect::<Vec<String>>();
        let extensions_supported =runtime_keys["watch"]["extensions_supported"].as_str().unwrap().to_string().split(",")
            .map(|file|{
                file.to_string()
            }).collect::<Vec<String>>();

        Runtime{
            split_files: SplitFiles {
                enable: runtime_keys["split_files"]["enable"].as_bool().unwrap(),
                amount_parts: runtime_keys["split_files"]["amount_parts"].as_i64().unwrap() as u8
        },
            watch: Watch {
                folders: folders,
                extensions_supported: extensions_supported 
            } 
        }
    }
}



fn load_file_config()-> Yaml {
    let path = env::current_dir().unwrap();
    let config_path =path.join("resources/config.yaml");

    let file_result = File::open(config_path.as_os_str());

    let file = match file_result {
        Ok(mut file)=>{
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            content
        },
        Err(e)=> {
            panic!("Not load config {}", e)
        }
    };
    let docs = YamlLoader::load_from_str(&file).unwrap();
    docs[0].to_owned()
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_config() {
        let config  = Config::get_config();
        let aws = config.get_aws_config();
        assert_eq!(aws.is_some(), true);
        let aws_config = aws.unwrap();
        assert_eq!(aws_config.bucket, "xxx");
    }
}