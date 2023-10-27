use std::path::PathBuf;

extern crate dotenv;

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum AppEnvironment {
    Dev,
    Test,
    Prod,
}

#[allow(unused)]
pub fn load_environment_variables(app_env: AppEnvironment) -> PathBuf {
    let dotenv_file = match app_env {
        AppEnvironment::Dev => ".env.dev",
        AppEnvironment::Test => ".env.test",
        AppEnvironment::Prod => ".env.prod",
    };

    dotenv::from_filename(dotenv_file)
        .ok()
        .expect("Failed to load environment variables.")
}
