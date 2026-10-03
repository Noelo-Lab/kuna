//! Choose the C spelling of an unconditional loop.

use kuna_base::error::{KunaError, KunaResult};

pub fn parse_inf_loop_style(value: &str) -> KunaResult<(bool, String)> {
    let top = match value {
        "while" => true,
        "do" => false,
        other => return Err(KunaError::parse(format!(
            "Unknown infloopstyle value: {other} (expected while|do)"
        ))),
    };
    Ok((top, format!("Infinite loop style set to {value}")))
}
