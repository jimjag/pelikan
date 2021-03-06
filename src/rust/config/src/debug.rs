// Copyright 2020 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use log::Level;
use serde::{Deserialize, Serialize};

// constants to define default values
const DEBUG_LOG_LEVEL: Level = Level::Info;
const DEBUG_LOG_FILE: Option<String> = None;
const DEBUG_LOG_NBUF: usize = 0;

// helper functions
fn log_level() -> Level {
    DEBUG_LOG_LEVEL
}

fn log_file() -> Option<String> {
    DEBUG_LOG_FILE
}

fn log_nbuf() -> usize {
    DEBUG_LOG_NBUF
}

// struct definitions
#[derive(Serialize, Deserialize, Debug)]
pub struct DebugConfig {
    #[serde(with = "LevelDef")]
    #[serde(default = "log_level")]
    log_level: Level,
    #[serde(default = "log_file")]
    log_file: Option<String>,
    #[serde(default = "log_nbuf")]
    log_nbuf: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[serde(remote = "Level")]
#[serde(deny_unknown_fields)]
enum LevelDef {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

// implementation
impl DebugConfig {
    pub fn log_level(&self) -> Level {
        self.log_level
    }

    pub fn log_file(&self) -> Option<String> {
        self.log_file.clone()
    }

    pub fn log_nbuf(&self) -> usize {
        self.log_nbuf
    }
}

// trait implementations
impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            log_level: log_level(),
            log_file: log_file(),
            log_nbuf: log_nbuf(),
        }
    }
}
