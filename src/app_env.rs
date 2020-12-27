pub fn check_env() {
    get_env::pwd_salt();
    get_env::session_key();
    get_env::port();
    get_env::gcp_project();
}

pub mod get_env {
    use std::env::var;

    fn extract_env<'a>(name: &'a str) -> String {
        var(name).expect(format!("{} variable is unset", name).as_str())
    }

    pub fn pwd_salt() -> String {
        extract_env("PWD_SALT")
    }

    pub fn session_key() -> String {
        extract_env("SESSION_KEY")
    }

    pub fn port() -> String {
        extract_env("PORT")
    }

    pub fn gcp_project() -> String {
        extract_env("GCP_PROJECT")
    }

    pub fn datastore_emulator_host() -> Option<String> {
        match var("DATASTORE_EMULATOR_HOST") {
            Ok(host) if host.len() > 2 => Some(host),
            _ => None,
        }
    }
}
