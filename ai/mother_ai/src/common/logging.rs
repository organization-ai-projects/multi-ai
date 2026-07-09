//! Module de logging
//!
//! Fournit des fonctionnalités de logging améliorées avec différents niveaux
//! et possibilité de filtrage.

use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::Display;

/// Niveaux de log supportés
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// Structure pour configurer et utiliser le système de logging
pub struct Logger {
    min_level: LogLevel,
    output_console: bool,
    context: String,
}

impl Logger {
    /// Crée un nouveau logger avec un niveau minimum et un contexte
    pub fn new(min_level: LogLevel, context: &str) -> Self {
        Self {
            min_level,
            output_console: true,
            context: context.to_string(),
        }
    }

    /// Désactive la sortie console
    pub fn disable_console(&mut self) -> &mut Self {
        self.output_console = false;
        self
    }

    /// Enregistre un message si le niveau est suffisant
    pub fn log(&self, level: LogLevel, message: &str) {
        if level >= self.min_level {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis();
            
            let formatted = format!(
                "[{} | {} | {}] {}",
                timestamp,
                level,
                self.context,
                message
            );
            
            if self.output_console {
                match level {
                    LogLevel::Debug | LogLevel::Info => println!("{}", formatted),
                    LogLevel::Warning => println!("\x1b[33m{}\x1b[0m", formatted), // Jaune
                    LogLevel::Error | LogLevel::Critical => eprintln!("\x1b[31m{}\x1b[0m", formatted), // Rouge
                }
            }
            
            // Ici on pourrait ajouter l'écriture dans un fichier de log
            // ou l'envoi à un service de monitoring
        }
    }
    
    /// Log niveau debug
    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }
    
    /// Log niveau info
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }
    
    /// Log niveau warning
    pub fn warning(&self, message: &str) {
        self.log(LogLevel::Warning, message);
    }
    
    /// Log niveau error
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
    
    /// Log niveau critical
    pub fn critical(&self, message: &str) {
        self.log(LogLevel::Critical, message);
    }
}

/// Logger global partagé entre les différents composants
static GLOBAL_LOGGER: once_cell::sync::Lazy<Arc<Mutex<Logger>>> = once_cell::sync::Lazy::new(|| {
    Arc::new(Mutex::new(Logger::new(LogLevel::Info, "GLOBAL")))
});

/// Configure le logger global
pub fn configure_global_logger(min_level: LogLevel, context: &str) {
    if let Ok(mut logger) = GLOBAL_LOGGER.lock() {
        *logger = Logger::new(min_level, context);
    }
}

/// Accède au logger global pour enregistrer un message
pub fn log(level: LogLevel, message: &str) {
    if let Ok(logger) = GLOBAL_LOGGER.lock() {
        logger.log(level, message);
    }
}

/// Macros pour faciliter le logging
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::common::logging::log($crate::common::logging::LogLevel::Debug, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::common::logging::log($crate::common::logging::LogLevel::Info, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)*) => {
        $crate::common::logging::log($crate::common::logging::LogLevel::Warning, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::common::logging::log($crate::common::logging::LogLevel::Error, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_critical {
    ($($arg:tt)*) => {
        $crate::common::logging::log($crate::common::logging::LogLevel::Critical, &format!($($arg)*))
    };
}
