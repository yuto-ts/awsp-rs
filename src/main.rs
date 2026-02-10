use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

use dialoguer::FuzzySelect;
use dialoguer::theme::ColorfulTheme;

const DEFAULT_PROFILE: &str = "default";

fn read_profiles(config_path: &std::path::Path) -> Result<Vec<String>, String> {
    let content = fs::read_to_string(config_path)
        .map_err(|e| format!("Failed to read {}: {}", config_path.display(), e))?;

    let mut profiles = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[default]" {
            profiles.push(DEFAULT_PROFILE.to_string());
        } else if let Some(rest) = trimmed.strip_prefix("[profile ") {
            if let Some(name) = rest.strip_suffix(']') {
                let name = name.trim();
                if !name.is_empty() {
                    profiles.push(name.to_string());
                }
            }
        }
    }

    Ok(profiles)
}

fn main() {
    let config_path = dirs::home_dir()
        .expect("Could not determine home directory")
        .join(".aws")
        .join("config");

    if !config_path.exists() {
        eprintln!("AWS config file not found: {}", config_path.display());
        eprintln!("Run `aws configure` to set up your AWS profiles.");
        process::exit(1);
    }

    let profiles = match read_profiles(&config_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if profiles.is_empty() {
        eprintln!("No profiles found in {}", config_path.display());
        eprintln!("Run `aws configure --profile PROFILE_NAME` to create a profile.");
        process::exit(1);
    }

    // Show current profile on stderr
    let current = env::var("AWS_PROFILE").unwrap_or_else(|_| DEFAULT_PROFILE.to_string());
    eprintln!("AWS Profile Switcher");
    eprintln!("Current profile: {}\n", current);

    // Find default selection index
    let default_index = profiles
        .iter()
        .position(|p| p == &current)
        .unwrap_or(0);

    // FuzzySelect writes UI to stderr by default
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a profile")
        .items(&profiles)
        .default(default_index)
        .interact_opt();

    match selection {
        Ok(Some(index)) => {
            let selected = &profiles[index];
            // Output shell command to stdout for eval
            let stdout = io::stdout();
            let mut out = stdout.lock();
            if selected == DEFAULT_PROFILE {
                writeln!(out, "unset AWS_PROFILE").ok();
            } else {
                writeln!(out, "export AWS_PROFILE={}", selected).ok();
            }
            eprintln!("\nSwitched to profile: {}", selected);
        }
        Ok(None) | Err(_) => {
            // Ctrl+C or error: output nothing to stdout
        }
    }
}
