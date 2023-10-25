use std::{env, path::PathBuf};

extern crate dotenv;

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum AppEnvironment {
    Dev,
    Test,
    Prod,
}

#[allow(unused)]
pub fn load_environment_variables(app_env: Option<AppEnvironment>) -> PathBuf {
    let app_env = app_env.unwrap_or_else(|| {
        let env_var =
            env::var("APP_ENV").expect("You must specify an APP_ENV (dev, test, or prod).");
        match env_var.as_str() {
            "dev" => AppEnvironment::Dev,
            "prod" => AppEnvironment::Prod,
            "test" => AppEnvironment::Test,
            _ => panic!(
                "Unrecognized APP_ENV {:?}. Must provide dev, test, or prod.",
                env_var
            ),
        }
    });

    let dotenv_file = match app_env {
        AppEnvironment::Dev => ".env.dev",
        AppEnvironment::Test => ".env.test",
        AppEnvironment::Prod => ".env.prod",
    };

    dotenv::from_filename(dotenv_file)
        .ok()
        .expect("Failed to load environment variables.")
}
