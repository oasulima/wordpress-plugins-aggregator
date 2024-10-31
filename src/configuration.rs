#[derive(serde::Deserialize, Debug, Clone)]
pub struct Settings {
    pub a1: Vec<A1Client>,
    pub mts: Vec<MTSClient>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct A1Client {
    pub title: String,
    pub username: String,
    pub password: String,
    pub billing_account: String,
    pub account_type: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct MTSClient {
    pub title: String,
    pub username: String,
    pub password: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("app.yaml")))
        .build()?;

    let result = settings.try_deserialize::<Settings>()?;

    Ok(result)
}
