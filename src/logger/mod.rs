//! # Warhammoger Logger
//!
//! Библиотека логирования в стиле Warhammer 40,000: красивый цветной вывод
//! в терминал и параллельная запись в файл журнала.
//!
//! ## Быстрый старт
//!
//! ```no_run
//! use warhammoger::logger;
//!
//! fn main() {
//!     // Необязательно: без init() используется конфигурация по умолчанию
//!     // (папка ./logs, уровень Trace, цветной вывод).
//!     logger::info("Легион выступил в поход");
//!     logger::warn("Пробит щит ордена");
//!     logger::error("Контакт с силами Хаоса потерян");
//! }
//! ```
//!
//! ## Своя конфигурация
//!
//! ```no_run
//! use warhammoger::logger::{self, LoggerConfig, Level};
//!
//! logger::init(
//!     LoggerConfig::new()
//!         .with_app_name("ADEPTUS")
//!         .with_log_dir("var/logs")
//!         .with_min_level(Level::Info),
//! );
//!
//! logger::info("Инициализация завершена");
//! ```

mod config;
mod level;
mod writer;

pub use config::LoggerConfig;
pub use level::Level;

use chrono::Local;
use once_cell::sync::OnceCell;
use writer::FileWriter;

struct Logger {
    config: LoggerConfig,
    file_writer: Option<FileWriter>,
}

static LOGGER: OnceCell<Logger> = OnceCell::new();

fn logger() -> &'static Logger {
    LOGGER.get_or_init(|| build_logger(LoggerConfig::default()))
}

fn build_logger(config: LoggerConfig) -> Logger {
    let file_writer = if config.write_to_file {
        match FileWriter::new(&config.log_dir, &config.app_name) {
            Ok(w) => Some(w),
            Err(err) => {
                eprintln!(
                    "[WARHAMMOGER] Не удалось открыть файл журнала в '{}': {err}",
                    config.log_dir.display()
                );
                None
            }
        }
    } else {
        None
    };

    Logger {
        config,
        file_writer,
    }
}

/// Инициализирует логгер заданной конфигурацией.
///
/// Должна быть вызвана один раз, до первого использования `info`/`warn`/...
/// Повторный вызов или вызов после первого лог-сообщения ни на что не влияет
/// (логгер уже создан) — в этом случае функция возвращает `false`.
pub fn init(config: LoggerConfig) -> bool {
    LOGGER.set(build_logger(config)).is_ok()
}

/// Короткая форма [`init`] с конфигурацией по умолчанию.
pub fn init_default() -> bool {
    init(LoggerConfig::default())
}

fn log(level: Level, message: &str) {
    let logger = logger();

    if level < logger.config.min_level {
        return;
    }

    let now = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    let app = &logger.config.app_name;

    let plain_line = format!(
        "[{now}] [{level}] [{title}] {app} :: {message}",
        level = level.as_str(),
        title = level.title(),
    );

    if logger.config.colored_terminal {
        let color = level.ansi_color();
        let reset = "\x1b[0m";
        let bold = "\x1b[1m";
        println!(
            "{color}[{now}] {icon} {bold}[{lvl:<5}]{reset}{color} [{title}] {app} :: {message}{reset}",
            icon = level.icon(),
            lvl = level.as_str(),
            title = level.title(),
        );
    } else {
        println!("{plain_line}");
    }

    if let Some(writer) = &logger.file_writer {
        if let Err(err) = writer.write_line(&plain_line) {
            eprintln!("[WARHAMMOGER] Ошибка записи в файл журнала: {err}");
        }
    }
}

/// Самый тихий уровень — для детальной трассировки выполнения.
pub fn trace(message: impl AsRef<str>) {
    log(Level::Trace, message.as_ref());
}

/// Отладочные сведения для разработчика.
pub fn debug(message: impl AsRef<str>) {
    log(Level::Debug, message.as_ref());
}

/// Обычное информационное сообщение — "глашатай" докладывает об успехе.
pub fn info(message: impl AsRef<str>) {
    log(Level::Info, message.as_ref());
}

/// Предупреждение — "недоброе предзнаменование", требующее внимания.
pub fn warn(message: impl AsRef<str>) {
    log(Level::Warn, message.as_ref());
}

/// Ошибка — "ересь", прерывающая нормальный ход выполнения.
pub fn error(message: impl AsRef<str>) {
    log(Level::Error, message.as_ref());
}
