// <FILE>tui-vfx-debug/src/logger.rs</FILE> - <DESC>Core centralized debug logger with global singleton and log history</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>WG3: Debug Logger Integration</WCTX>
// <CLOG>Initial creation with global singleton, color output, log history, and JSON export</CLOG>

use crate::config::{LogLevel, ModuleConfig, module_registry};
use chrono::Local;
use colored::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub namespace: String,
    pub level: String,
    pub message: String,
}

pub struct DebugLogger {
    enabled_namespaces: Arc<Mutex<HashSet<String>>>,
    log_history: Arc<Mutex<Vec<LogEntry>>>,
    module_registry: HashMap<String, ModuleConfig>,
}

impl Default for DebugLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugLogger {
    pub fn new() -> Self {
        let module_registry = module_registry();
        let enabled_namespaces = Arc::new(Mutex::new(HashSet::new()));

        // Auto-enable modules set to Debug or Info
        for (namespace, config) in &module_registry {
            if matches!(config.level, LogLevel::Debug | LogLevel::Info) {
                enabled_namespaces.lock().unwrap().insert(namespace.clone());
            }
        }

        DebugLogger {
            enabled_namespaces,
            log_history: Arc::new(Mutex::new(Vec::new())),
            module_registry,
        }
    }

    pub fn enable(&self, namespace: &str) {
        self.enabled_namespaces
            .lock()
            .unwrap()
            .insert(namespace.to_string());
    }

    pub fn disable(&self, namespace: &str) {
        self.enabled_namespaces.lock().unwrap().remove(namespace);
    }

    pub fn is_enabled(&self, namespace: &str) -> bool {
        self.enabled_namespaces.lock().unwrap().contains(namespace)
    }

    pub fn get_level(&self, namespace: &str) -> LogLevel {
        self.module_registry
            .get(namespace)
            .map(|config| config.level)
            .unwrap_or(LogLevel::Off)
    }

    pub fn log(&self, namespace: &str, level: &str, message: &str) {
        if !self.is_enabled(namespace) {
            return;
        }

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let entry = LogEntry {
            timestamp: timestamp.clone(),
            namespace: namespace.to_string(),
            level: level.to_string(),
            message: message.to_string(),
        };

        // Store in history
        self.log_history.lock().unwrap().push(entry);

        // Color-coded console output
        let colored_level = match level {
            "DEBUG" => level.blue(),
            "INFO" => level.green(),
            "WARN" => level.yellow(),
            "ERROR" => level.red(),
            _ => level.normal(),
        };

        println!(
            "[{}] {} [{}] {}",
            timestamp.dimmed(),
            colored_level,
            namespace.cyan(),
            message
        );
    }

    pub fn get_history(&self) -> Vec<LogEntry> {
        self.log_history.lock().unwrap().clone()
    }

    pub fn clear_history(&self) {
        self.log_history.lock().unwrap().clear();
    }

    pub fn export_to_json(&self) -> Result<String, serde_json::Error> {
        let history = self.get_history();
        serde_json::to_string_pretty(&history)
    }
}

lazy_static! {
    static ref GLOBAL_LOGGER: Arc<Mutex<Option<Arc<DebugLogger>>>> =
        Arc::new(Mutex::new(Some(Arc::new(DebugLogger::new()))));
}

pub fn get_global_logger() -> Option<Arc<DebugLogger>> {
    GLOBAL_LOGGER.lock().unwrap().clone()
}

pub fn create_logger(namespace: &str) -> Logger {
    Logger {
        namespace: namespace.to_string(),
    }
}

pub struct Logger {
    namespace: String,
}

impl Logger {
    pub fn debug(&self, message: &str) {
        if let Some(logger) = get_global_logger() {
            if matches!(logger.get_level(&self.namespace), LogLevel::Debug) {
                logger.log(&self.namespace, "DEBUG", message);
            }
        }
    }

    pub fn info(&self, message: &str) {
        if let Some(logger) = get_global_logger() {
            let level = logger.get_level(&self.namespace);
            if matches!(level, LogLevel::Debug | LogLevel::Info) {
                logger.log(&self.namespace, "INFO", message);
            }
        }
    }

    pub fn warn(&self, message: &str) {
        if let Some(logger) = get_global_logger() {
            let level = logger.get_level(&self.namespace);
            if matches!(level, LogLevel::Debug | LogLevel::Info | LogLevel::Warn) {
                logger.log(&self.namespace, "WARN", message);
            }
        }
    }

    pub fn error(&self, message: &str) {
        if let Some(logger) = get_global_logger() {
            let level = logger.get_level(&self.namespace);
            if !matches!(level, LogLevel::Off) {
                logger.log(&self.namespace, "ERROR", message);
            }
        }
    }
}

// <FILE>tui-vfx-debug/src/logger.rs</FILE> - <DESC>Core centralized debug logger with global singleton and log history</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
