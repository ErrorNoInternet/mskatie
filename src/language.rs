use phf::phf_map;

use crate::logging::{log_message, LogMessageType};

static TEXT: phf::Map<&'static str, &'static str> = phf_map! {
    "pong" => "🏓 Pong!",
};

pub fn get_text(key: &str) -> &str {
    match TEXT.get(key) {
        Some(value) => value,
        None => {
            log_message(
                LogMessageType::Error,
                &format!("Missing text for \"{key}\"!"),
            );
            "?"
        }
    }
}
