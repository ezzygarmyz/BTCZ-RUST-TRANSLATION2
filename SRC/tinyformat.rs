use std::fmt;

/// A lightweight formatter for building formatted strings
pub struct TinyFormatter;

impl TinyFormatter {
    /// Formats a string with positional and named arguments (mimics printf style)
    pub fn format(fmt: &str, args: &[&dyn fmt::Display]) -> String {
        let mut formatted = String::new();
        let mut chars = fmt.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '%' {
                match chars.peek() {
                    Some('%') => {
                        formatted.push('%');
                        chars.next(); // Consume the second '%'
                    }
                    _ => {
                        // Use the next available argument
                        if let Some(arg) = args.get(formatted.len()) {
                            formatted.push_str(&arg.to_string());
                        } else {
                            formatted.push_str("<missing>");
                        }
                    }
                }
            } else {
                formatted.push(ch);
            }
        }

        formatted
    }

    /// Formats a string with named arguments using Rust's native formatting
    pub fn named_format(fmt: &str, named_args: &dyn fmt::Debug) -> String {
        format!("{}", format_args!(fmt, named_args = named_args))
    }
}

/// A trait for implementing custom formatting
pub trait TinyFormat: fmt::Display {
    fn tiny_format(&self) -> String {
        format!("{}", self)
    }
}

impl<T: fmt::Display> TinyFormat for T {}
