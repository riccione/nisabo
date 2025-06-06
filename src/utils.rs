use std::fmt::Display;
use crate::constants::RESULT_SUCCESS;

pub fn result<E: Display>(result: Result<(), E>, msg: &str) -> String {
    match result {
        Ok(()) => String::from(RESULT_SUCCESS),
        Err(e) => format!("{}: {}", msg, e),
    }
}
