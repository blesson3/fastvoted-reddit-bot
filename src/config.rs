use serde::Deserialize;
use std::{fs, path::Path};

const CONFIG_JSON_FILENAME: &str = "config.json";
const LAST_POSTED_ID_FILENAME: &str = "last-posted-id.txt";

#[derive(Debug, Deserialize)]
pub struct Config
{
    pub username:       String,
    pub password:       String,
    pub client_id:      String,
    pub client_secret:  String,
    pub subreddit_name: String,

    // this is set after the Config instance is instantiated
    #[serde(default)]
    pub config_dir: String,
}

impl Config
{
    pub fn from_directory(config_dir: &Path) -> Config
    {
        let config_filepath = config_dir.join(CONFIG_JSON_FILENAME);

        let data = fs::read_to_string(&config_filepath).expect(&format!(
            "unable to read config file at {:?}",
            &config_filepath
        ));
        let mut config: Config = serde_json::from_str(&data).expect(&format!(
            "unable to deserialize contents of config file at {:?}",
            &config_filepath
        ));

        // set the config dir
        config.config_dir = config_dir.display().to_string();
        config
    }

    pub fn get_last_posted_id_filepath(&self) -> String
    {
        let path = Path::new(&self.config_dir).join(&LAST_POSTED_ID_FILENAME);
        path.display().to_string()
    }
}
