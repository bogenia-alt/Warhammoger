use std::fmt;

/// Боевой ранг записи журнала — определяет цвет, значок и порог вывода.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Level {
    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Trace => "TRACE",
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
        }
    }

    /// Титул в стиле Imperium, используемый в оформлении терминала.
    pub fn title(&self) -> &'static str {
        match self {
            Level::Trace => "WHISPER",
            Level::Debug => "SCRIBE",
            Level::Info => "HERALD",
            Level::Warn => "OMEN",
            Level::Error => "HERESY",
        }
    }

    /// ANSI-код цвета для терминального вывода.
    pub fn ansi_color(&self) -> &'static str {
        match self {
            Level::Trace => "\x1b[90m",   // серый
            Level::Debug => "\x1b[36m",   // голубой
            Level::Info => "\x1b[32m",    // зелёный
            Level::Warn => "\x1b[33m",    // жёлтый
            Level::Error => "\x1b[31;1m", // ярко-красный
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Level::Trace => "·",
            Level::Debug => "🔧",
            Level::Info => "⚜",
            Level::Warn => "☠",
            Level::Error => "🔥",
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
