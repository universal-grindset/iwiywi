#[derive(Debug, PartialEq)]
pub enum Action {
    ToggleQr,
    Unknown,
}

pub fn handle_command(cmd: &str) -> Action {
    match cmd.trim().to_lowercase().as_str() {
        "qr" => Action::ToggleQr,
        _ => Action::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qr_command_returns_toggle_qr() {
        assert_eq!(handle_command("qr"), Action::ToggleQr);
    }

    #[test]
    fn unknown_command_returns_unknown() {
        assert_eq!(handle_command("foo"), Action::Unknown);
    }

    #[test]
    fn command_is_trimmed() {
        assert_eq!(handle_command("  qr  "), Action::ToggleQr);
    }

    #[test]
    fn qr_command_case_insensitive() {
        assert_eq!(handle_command("QR"), Action::ToggleQr);
        assert_eq!(handle_command("Qr"), Action::ToggleQr);
        assert_eq!(handle_command("qR"), Action::ToggleQr);
    }
}
