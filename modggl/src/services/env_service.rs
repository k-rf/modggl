use std::env;

pub fn toggl_api() -> String {
    env::var("TOGGL_API").unwrap()
}

pub fn report_api() -> String {
    env::var("REPORT_API").unwrap()
}

pub fn workspace_id() -> String {
    env::var("WORKSPACE_ID").unwrap()
}

pub fn user_agent() -> String {
    env::var("USER_AGENT").unwrap()
}

pub fn token() -> String {
    env::var("TOKEN").unwrap()
}

pub fn token_type() -> String {
    env::var("TOKEN_TYPE").unwrap()
}
