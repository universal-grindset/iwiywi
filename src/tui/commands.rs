#[derive(Debug, PartialEq)]
pub enum Action {
    ToggleQr,
    Unknown,
}

pub fn handle_command(cmd: &str) -> Action {
    match cmd.trim() {
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
}
