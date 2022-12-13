use std::io;

use ansi_term::Colour as Color;
use regex::Regex;

const RX_STR: &str = r"^.+(DEBUG|INFO|TRACE) (.+)] ?(Program log:|Program|process_instruction:|solana_runtime:)? (.+)$";

#[derive(Debug)]
enum Importance {
    Low,
    Medium,
    High,
    VeryHigh,
    Error,
}

fn get_importance(
    log_level: &str,
    log_source: &str,
    program_source: &str,
    program_log: &str,
) -> Importance {
    let log = program_log.to_lowercase();
    if log.contains("error: ")
        || log.contains("error ")
        || log.contains("err: ")
        || log.contains("err ")
        || log.contains("failure: ")
        || log.contains("failure ")
        || log.contains("failed: ")
        || log.contains("failed ")
        || log.contains("fail: ")
        || log.contains("fail ")
    {
        return Importance::Error;
    }
    match log_level {
        "INFO" => Importance::VeryHigh,
        "DEBUG" if program_log.contains("signer privilege escalated") => {
            Importance::High
        }
        "DEBUG" if program_source == "Program log:" => Importance::VeryHigh,
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

fn colorize(importance: Importance, log_level: &str, log: &str) -> String {
    use Importance::*;
    let level = color_log_level(log_level);
    match importance {
        Error => format!("{} {}", level, Color::Fixed(9).bold().paint(log)),
        VeryHigh => format!("{} {}", level, Color::Green.paint(log)),
        High => format!("{} {}", level, Color::Fixed(243).bold().paint(log)),
        Medium => format!("{} {}", level, Color::Fixed(243).paint(log)),
        Low => format!("{} {}", level, Color::Fixed(239).paint(log)),
    }
}

fn format_line(rx: &Regex, line: &str) -> String {
    match rx.captures(line) {
        Some(caps) => {
            let log_level = caps.get(1).unwrap().as_str();
            let log_source = caps.get(2).unwrap().as_str();
            let program_source_cap = caps.get(3);
            let program_source = program_source_cap
                .map(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            let program_log = caps.get(4).unwrap().as_str().to_string();

            let importance = get_importance(
                log_level,
                log_source,
                &program_source,
                &program_log,
            );

            let log = if program_source_cap.is_none()
                || program_source == "Program log:"
            {
                program_log
            } else {
                format!("{} {}", program_source, program_log)
            };
            colorize(importance, log_level, &log)
        }
        None => line.to_string(),
    }
}

fn main() {
    let rx = Regex::new(RX_STR).unwrap();

    for line in io::stdin().lines().flatten() {
        let formatted = format_line(&rx, &line);
        println!("{}", formatted);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_importance_no_log_source_signer_privilege_escalated() {
        let rx = Regex::new(RX_STR).unwrap();
        let line = "[2022-12-13T20:55:47.950831000Z DEBUG solana_runtime::message_processor::stable_log] Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS's signer privilege escalated";
        let formatted = format_line(&rx, line);
        eprintln!("{}", formatted);
        assert_eq!(
            formatted,
            "\u{1b}[34mDEBUG\u{1b}[0m \u{1b}[1;38;5;243mFg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS's signer privilege escalated\u{1b}[0m"
        );
    }
}
