use std::io;

use ansi_term::Colour as Color;
use regex::Regex;

#[derive(Debug)]
enum Importance {
    Low,
    Medium,
    High,
    Error,
}

fn get_importance(
    log_level: &str,
    log_source: &str,
    program_source: &str,
    program_log: &str,
) -> Importance {
    if program_log.contains("custom program error") {
        return Importance::Error;
    }
    match log_level {
        "INFO" => Importance::High,
        "DEBUG" if program_source == "Program log:" => Importance::High,
        "DEBUG" if log_source.ends_with("stable_log") => Importance::Medium,
        "DEBUG" => Importance::Medium,
        "TRACE" => Importance::Low,
        _ => Importance::Low,
    }
}

fn color_log_level(log_level: &str) -> String {
    match log_level {
        "INFO" => Color::Green.paint(log_level).to_string(),
        "DEBUG" => Color::Blue.paint(log_level).to_string(),
        "TRACE" => Color::Fixed(239).paint(log_level).to_string(),
        _ => log_level.to_string(),
    }
}

fn log_pretty(importance: Importance, log_level: &str, log: &str) {
    use Importance::*;
    let level = color_log_level(log_level);
    match importance {
        Error => println!("{} {}", level, Color::Fixed(9).bold().paint(log)),
        High => println!("{} {}", level, Color::Green.paint(log)),
        Medium => println!("{} {}", level, Color::Fixed(243).paint(log)),
        Low => println!("{} {}", level, Color::Fixed(239).paint(log)),
    }
}

fn log_line(rx: &Regex, line: &str) {
    match rx.captures(line) {
        Some(caps) => {
            let log_level = caps.get(1).unwrap().as_str();
            let log_source = caps.get(2).unwrap().as_str();
            let program_source = caps.get(3).unwrap().as_str();
            let program_log = caps.get(4).unwrap().as_str();
            let importance = get_importance(
                log_level,
                log_source,
                program_source,
                program_log,
            );
            let log = format!("{} {}", program_source, program_log);
            log_pretty(importance, log_level, &log);
        }
        None => println!("{}", line),
    }
}

fn main() {
    let rx = Regex::new(
        r"^.+(DEBUG|INFO|TRACE) (.+)] (Program log:|Program|process_instruction:) (.+)$",
    )
    .unwrap();

    for line in io::stdin().lines().flatten() {
        log_line(&rx, &line)
    }
}
