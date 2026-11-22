//! Exact legacy Rust runtime contracts missing from the vendored no-return list.

pub(super) fn process_exit(name: &str) -> bool {
    let Some(tail) = name.trim_start_matches('_').strip_prefix("ZN3std7process4exit17h") else {
        return false;
    };
    let Some(hash) = tail.strip_suffix('E') else {
        return false;
    };
    hash.len() == 16 && hash.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

#[cfg(test)]
mod tests {
    use super::process_exit;

    #[test]
    fn rust_process_exit_requires_the_complete_legacy_identity() {
        assert!(process_exit("_ZN3std7process4exit17h644d5313113857bcE"));
        assert!(process_exit("ZN3std7process4exit17h0123456789abcdefE"));
        for name in [
            "exit",
            "std::process::exit",
            "_ZN3std7process11exit_status17h0123456789abcdefE",
            "_ZN4user7process4exit17h0123456789abcdefE",
            "_ZN3std7process4exit17hE",
            "_ZN3std7process4exit17h0123456789abcdeE",
            "_ZN3std7process4exit17h0123456789abcdeFE",
            "_ZN3std7process4exit17h0123456789abcdefE_suffix",
            "_ZN3std7process4exit17h0123456789abcdef",
        ] {
            assert!(!process_exit(name), "{name}");
        }
    }
}
