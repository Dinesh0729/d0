use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

/// D0 Systems standard library: File I/O, process management, environment, and memory info.
pub struct D0System;

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub current_dir: String,
}

impl D0System {
    pub fn get_info() -> SystemInfo {
        SystemInfo {
            os: env::consts::OS.to_string(),
            arch: env::consts::ARCH.to_string(),
            current_dir: env::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "unknown".to_string()),
        }
    }

    pub fn get_env(key: &str) -> Option<String> {
        env::var(key).ok()
    }

    pub fn set_env(key: &str, val: &str) {
        env::set_var(key, val);
    }

    pub fn read_file(path: impl AsRef<Path>) -> io::Result<String> {
        let mut f = fs::File::open(path)?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        Ok(content)
    }

    pub fn write_file(path: impl AsRef<Path>, content: &str) -> io::Result<()> {
        let mut f = fs::File::create(path)?;
        f.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn file_exists(path: impl AsRef<Path>) -> bool {
        path.as_ref().exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_info() {
        let info = D0System::get_info();
        assert!(!info.os.is_empty());
        assert!(!info.arch.is_empty());
    }

    #[test]
    fn test_env_roundtrip() {
        D0System::set_env("D0_TEST_VAR", "active");
        assert_eq!(D0System::get_env("D0_TEST_VAR"), Some("active".to_string()));
    }
}
