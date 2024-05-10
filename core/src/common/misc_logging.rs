use log::{log, Level};
use paris::formatter::colorize_string;

fn level_to_color(level: Level) -> &'static str {
    match level {
        Level::Info => "blue",
        Level::Warn => "yellow",
        Level::Error => "red",
        _ => "white",
    }
}

/// A BIG log that's very difficult to miss.
pub fn basti_log(level: Level, message: &str) {
    let color = level_to_color(level);
    log!(
        level,
        "{}",
        colorize_string(format!(
            "<bold><{}>{}\n\n",
            &color,
            "-".repeat(message.len())
        ))
    );
    log!(
        level,
        "{}",
        colorize_string(format!("<bold><{}>{}\n\n", &color, message))
    );
    log!(
        level,
        "{}",
        colorize_string(format!(
            "<bold><{}>{}\n\n",
            &color,
            "-".repeat(message.len())
        ))
    );
}
