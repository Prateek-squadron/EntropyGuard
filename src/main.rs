use std::io;

struct Password {
    value: String,
}

impl Password {
    fn new(value: String) -> Self {
        Self { value }
    }

    fn length(&self) -> usize {
        self.value.len()
    }

    fn has_uppercase(&self) -> bool {
        self.value.chars().any(|c| c.is_uppercase())
    }

    fn has_number(&self) -> bool {
        self.value.chars().any(|c| c.is_numeric())
    }

    fn has_special(&self) -> bool {
        self.value.chars().any(|c| !c.is_alphanumeric())
    }

    fn charset_size(&self) -> u32 {
        let mut size = 0;

        if self.value.chars().any(|c| c.is_lowercase()) {
            size += 26;
        }

        if self.value.chars().any(|c| c.is_uppercase()) {
            size += 26;
        }

        if self.value.chars().any(|c| c.is_numeric()) {
            size += 10;
        }

        if self.value.chars().any(|c| !c.is_alphanumeric()) {
            size += 32;
        }

        size
    }

    fn entropy(&self) -> f64 {
        let charset = self.charset_size() as f64;
        let length = self.length() as f64;

        length * charset.log2()
    }
    fn contains_pattern(&self) -> bool {
        let patterns = [
            "123", "1234", "abc", "abcd", "qwerty", "asdf", "password", "admin",
        ];

        for pattern in patterns {
            if self.value.to_lowercase().contains(pattern) {
                return true;
            }
        }

        false
    }
    fn risk_level(&self) -> &str {
        let entropy = self.entropy();

        if self.contains_pattern() {
            return "HIGH RISK";
        }

        if entropy < 40.0 {
            "WEAK"
        } else if entropy < 60.0 {
            "MODERATE"
        } else {
            "STRONG"
        }
    }
    fn total_combinations(&self) -> f64 {
        let charset = self.charset_size() as f64;
        let length = self.length() as f64;

        charset.powf(length)
    }
    fn crack_time_seconds(&self) -> f64 {
        let guesses_per_second = 10_000_000_000.0;

        self.total_combinations() / guesses_per_second
    }
    fn crack_time_display(&self) -> String {
        let seconds = self.crack_time_seconds();

        if seconds < 60.0 {
            format!("{:.2} seconds", seconds)
        } else if seconds < 3600.0 {
            format!("{:.2} minutes", seconds / 60.0)
        } else if seconds < 86400.0 {
            format!("{:.2} hours", seconds / 3600.0)
        } else if seconds < 31536000.0 {
            format!("{:.2} days", seconds / 86400.0)
        } else {
            format!("{:.2} years", seconds / 31536000.0)
        }
    }
}

fn main() {
    println!("==============================");
    println!("EntropyGuard v1.0");
    println!("Password Risk Analyzer");
    println!("==============================");
    println!("Enter password:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let password = Password::new(input.trim().to_string());
    
    println!("Length: {}", password.length());
    println!("Contains uppercase: {}", password.has_uppercase());
    println!("Contains number: {}", password.has_number());
    println!("Contains special char: {}", password.has_special());
    println!("Entropy: {:.2} bits", password.entropy());
    println!("Contains weak pattern: {}", password.contains_pattern());
    println!("Risk level: {}", password.risk_level());
    println!("Estimated crack time: {}", password.crack_time_display());
}
