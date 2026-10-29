use super::level::Level;
use std::path::PathBuf;

/// Конфигурация библиотеки логирования.
///
/// Создаётся один раз и передаётся в [`crate::logger::init`]
/// до первого вызова `info`/`warn`/`error`/`debug`/`trace`.
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    /// Минимальный уровень, который будет напечатан и записан в файл.
    pub min_level: Level,
    /// Папка, в которую пишутся файлы журнала. По умолчанию — `./logs`.
    pub log_dir: PathBuf,
    /// Имя приложения/легиона, отображаемое в каждой строке.
    pub app_name: String,
    /// Включить цветной вывод в терминал.
    pub colored_terminal: bool,
    /// Включить запись в файл.
    pub write_to_file: bool,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            min_level: Level::Trace,
            log_dir: PathBuf::from("logs"),
            app_name: "WARHAMMOGER".to_string(),
            colored_terminal: true,
            write_to_file: true,
        }
    }
}

impl LoggerConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_min_level(mut self, level: Level) -> Self {
        self.min_level = level;
        self
    }

    pub fn with_log_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.log_dir = dir.into();
        self
    }

    pub fn with_app_name(mut self, name: impl Into<String>) -> Self {
        self.app_name = name.into();
        self
    }

    pub fn with_colored_terminal(mut self, enabled: bool) -> Self {
        self.colored_terminal = enabled;
        self
    }

    pub fn with_write_to_file(mut self, enabled: bool) -> Self {
        self.write_to_file = enabled;
        self
    }
}
