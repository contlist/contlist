use chrono::{Datelike, Utc};
use colored::{Color, Colorize};
use fern::Output;
use log::{Level, LevelFilter, Record};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

#[derive(Debug)]
pub struct FileTreeOutput {
    stream: Mutex<Option<BufWriter<File>>>,
    past_day: AtomicU32,
    log_root: PathBuf,
}

impl FileTreeOutput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_dir(&self) {
        let now = Utc::now();
        let current_day = now.day();

        if current_day == self.past_day.load(Ordering::SeqCst) {
            return;
        }

        self.past_day.store(current_day, Ordering::SeqCst);

        let dir_path = self.log_root.join(now.format("%Y/%B/").to_string());
        fs::create_dir_all(dir_path).expect("failed to create log dir");

        let file_path = self.log_root.join(now.format("%Y/%B/%F.log").to_string());
        let file = fern::log_file(file_path).expect("failed to create log file");

        let mut lock = self.stream.lock().unwrap_or_else(|e| e.into_inner());
        *lock = Some(BufWriter::new(file));
    }

    pub fn log(&self, record: &Record<'_>) {
        self.update_dir();

        let mut lock = self.stream.lock().unwrap_or_else(|e| e.into_inner());
        let writer = lock.as_mut().unwrap();

        write!(writer, "{}\n", record.args()).expect("failed to write log");
        writer.flush().expect("failed to flush log write");
    }

    pub fn log_root(&self) -> &Path {
        self.log_root.as_path()
    }
}

impl Default for FileTreeOutput {
    fn default() -> Self {
        Self {
            stream: Mutex::new(None),
            past_day: AtomicU32::new(0),
            log_root: Path::new("logs/").into(),
        }
    }
}

impl From<FileTreeOutput> for Output {
    fn from(tree: FileTreeOutput) -> Self {
        Self::call(move |record| tree.log(record))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LoggerConfig<'p> {
    file_release_level: LevelFilter,
    file_debug_level: LevelFilter,
    stdout_release_level: LevelFilter,
    stdout_debug_level: LevelFilter,
    log_root: &'p Path,
}

impl<'p> LoggerConfig<'p> {
    /// returns release level for file output
    pub fn file_release_level(&mut self, level: LevelFilter) -> &Self {
        self.file_release_level = level;
        self
    }

    /// returns debug level for file output
    pub fn file_debug_level(&mut self, level: LevelFilter) -> &Self {
        self.file_debug_level = level;
        self
    }

    /// return release level for std output
    pub fn stdout_release_level(&mut self, level: LevelFilter) -> &Self {
        self.stdout_release_level = level;
        self
    }

    /// return debug level for std output
    pub fn stdout_debug_level(&mut self, level: LevelFilter) -> &Self {
        self.stdout_debug_level = level;
        self
    }

    /// returns the directory of all logs
    pub fn log_root(&mut self, log_root: &'p impl AsRef<Path>) -> &Self {
        self.log_root = log_root.as_ref();
        self
    }

    /// initializes logger
    pub fn init(&mut self) -> Result<(), log::SetLoggerError> {
        let file_logger = fern::Dispatch::new()
            .level(self.current_file_level())
            .format(|out, msg, record| {
                let time = chrono::Local::now().format("%T%.3f");

                if record.target() == "_" {
                    out.finish(format_args!("{} => [{}] {}", time, record.level(), msg))
                } else {
                    out.finish(format_args!(
                        "{} - {} => [{}] {}",
                        time,
                        record.target(),
                        record.level(),
                        msg
                    ))
                };
            })
            .chain(FileTreeOutput::new());

        let stdout_logger = fern::Dispatch::new()
            .level(self.current_stdout_level())
            .format(|out, msg, record| {
                let colored_time = chrono::Local::now()
                    .format("%T%.3f")
                    .to_string()
                    .color(color_from_level(record.level()));

                if record.target() == "_" {
                    out.finish(format_args!("{} {} {}", colored_time, "=>".blue(), msg))
                } else {
                    out.finish(format_args!(
                        "{} - {} {} {}",
                        colored_time,
                        record.target().cyan(),
                        "=>".blue(),
                        msg
                    ))
                };
            })
            .chain(std::io::stdout());

        fern::Dispatch::new()
            .chain(file_logger)
            .chain(stdout_logger)
            .apply()
    }

    /// returns current filter for file logging depends on build mode (release or debug)
    #[cfg(debug_assertions)]
    fn current_file_level(&self) -> LevelFilter {
        self.file_debug_level
    }

    /// returns current filter for file logging depends on build mode (release or debug)
    #[cfg(not(debug_assertions))]
    fn current_file_level(&self) -> LevelFilter {
        self.file_release_level
    }

    /// returns current filter for stdout logging depends on build mode (release or debug)
    #[cfg(debug_assertions)]
    fn current_stdout_level(&self) -> LevelFilter {
        self.stdout_debug_level
    }

    /// returns current filter for stdout logging depends on build mode (release or debug)
    #[cfg(not(debug_assertions))]
    fn current_stdout_level(&self) -> LevelFilter {
        self.stdout_release_level
    }
}

impl Default for LoggerConfig<'_> {
    fn default() -> Self {
        Self {
            file_release_level: LevelFilter::Error,
            file_debug_level: LevelFilter::Debug,
            stdout_release_level: LevelFilter::Error,
            stdout_debug_level: LevelFilter::Debug,
            log_root: Path::new("logs/"),
        }
    }
}

/// Defines color for every logging level
fn color_from_level(level: Level) -> Color {
    match level {
        Level::Error => Color::Red,
        Level::Warn => Color::Yellow,
        Level::Info => Color::Green,
        Level::Debug => Color::White,
        Level::Trace => Color::Magenta,
    }
}

/// `ResultExt` trait defines and implements inspect metods
pub trait ResultInspect<T, E> {
    /// Does something with the contained `Ok` value
    ///
    /// Suggested usage for logging
    fn inspect(self, f: impl FnOnce(&T)) -> Self;

    /// Does something with the contained `Err` value
    ///
    /// Suggested usage for logging
    fn inspect_err(self, f: impl FnOnce(&E)) -> Self;
}

impl<T, E> ResultInspect<T, E> for Result<T, E> {
    fn inspect(self, f: impl FnOnce(&T)) -> Self {
        if let Ok(ref o) = self {
            (f)(o);
        }

        self
    }

    fn inspect_err(self, f: impl FnOnce(&E)) -> Self {
        if let Err(ref e) = self {
            (f)(e);
        }

        self
    }
}

/// `ResultExt` trait defines and implements inspect reference metods
pub trait ResultInspectRef<T, E> {
    /// Does something with the contained `Ok` value
    ///
    /// Suggested usage for logging
    fn inspect_ref(&self, f: impl FnOnce(&T)) -> &Self;

    /// Does something with the contained `Err` value
    ///
    /// Suggested usage for logging
    fn inspect_err_ref(&self, f: impl FnOnce(&E)) -> &Self;
}

impl<T, E> ResultInspectRef<T, E> for Result<T, E> {
    fn inspect_ref(&self, f: impl FnOnce(&T)) -> &Self {
        if let Ok(o) = self {
            (f)(o);
        }

        self
    }

    fn inspect_err_ref(&self, f: impl FnOnce(&E)) -> &Self {
        if let Err(e) = self {
            (f)(e);
        }

        self
    }
}
