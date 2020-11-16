pub fn check_env() {
    std::env::var("PWD_SALT").expect("PWD_SALT is unset");
    std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
        .expect("GOOGLE_APPLICATION_CREDENTIALS is unset");
    std::env::var("SESSION_KEY").expect("SESSION_KEY variable is unavailable");
}
