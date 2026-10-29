use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use chrono::Local;

/// Writer, пишущий записи журнала в файл `logs/<app>-<YYYY-MM-DD>.log`,
/// пересоздавая файл дня при смене даты.
pub struct FileWriter {
    inner: Mutex<FileWriterState>,
    dir: PathBuf,
    app_name: String,
}

struct FileWriterState {
    file: Option<File>,
    current_date: String,
}

impl FileWriter {
    pub fn new(dir: impl Into<PathBuf>, app_name: impl Into<String>) -> io::Result<Self> {
        let dir = dir.into();
        let app_name = app_name.into();
        fs::create_dir_all(&dir)?;

        let today = Local::now().format("%Y-%m-%d").to_string();
        let file = Some(Self::open_file(&dir, &app_name, &today)?);

        Ok(Self {
            inner: Mutex::new(FileWriterState {
                file,
                current_date: today,
            }),
            dir,
            app_name,
        })
    }

    fn open_file(dir: &Path, app_name: &str, date: &str) -> io::Result<File> {
        let path = dir.join(format!("{app_name}-{date}.log"));
        OpenOptions::new().create(true).append(true).open(path)
    }

    /// Записывает уже отформатированную (без ANSI-кодов) строку в файл дня.
    pub fn write_line(&self, line: &str) -> io::Result<()> {
        let mut state = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        let today = Local::now().format("%Y-%m-%d").to_string();
        if today != state.current_date || state.file.is_none() {
            state.file = Some(Self::open_file(&self.dir, &self.app_name, &today)?);
            state.current_date = today;
        }

        if let Some(file) = state.file.as_mut() {
            file.write_all(line.as_bytes())?;
            file.write_all(b"\n")?;
            file.flush()?;
        }

        Ok(())
    }
}
