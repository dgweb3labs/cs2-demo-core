use thiserror::Error;

/// Custom error types for CS2 demo parsing
#[derive(Error, Debug)]
pub enum DemoError {
    /// IO error when reading demo file
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Invalid demo file format
    #[error("Invalid demo format: {message}")]
    InvalidFormat { message: String },
    
    /// Demo file is corrupted or incomplete
    #[error("Corrupted demo file: {message}")]
    Corrupted { message: String },
    
    /// Unsupported demo version
    #[error("Unsupported demo version: {version}")]
    UnsupportedVersion { version: String },
    
    /// Protobuf parsing error
    #[error("Protobuf error: {0}")]
    Protobuf(#[from] protobuf::Error),
    
    /// JSON serialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    /// Invalid event data
    #[error("Invalid event data: {message}")]
    InvalidEvent { message: String },
    
    /// Demo file not found
    #[error("Demo file not found: {path}")]
    FileNotFound { path: String },
    
    /// Demo file is empty
    #[error("Demo file is empty")]
    EmptyFile,
    
    /// Timeout while parsing
    #[error("Parsing timeout after {timeout:?}")]
    Timeout { timeout: std::time::Duration },
}

/// Main result type for demo parsing operations
pub type Result<T> = std::result::Result<T, DemoError>;

impl DemoError {
    /// Create an invalid format error
    pub fn invalid_format(message: impl Into<String>) -> Self {
        Self::InvalidFormat {
            message: message.into(),
        }
    }
    
    /// Create a corrupted file error
    pub fn corrupted(message: impl Into<String>) -> Self {
        Self::Corrupted {
            message: message.into(),
        }
    }
    
    /// Create an invalid event error
    pub fn invalid_event(message: impl Into<String>) -> Self {
        Self::InvalidEvent {
            message: message.into(),
        }
    }
    
    /// Create a file not found error
    pub fn file_not_found(path: impl Into<String>) -> Self {
        Self::FileNotFound {
            path: path.into(),
        }
    }
    
    pub fn unsupported_version(version: impl Into<String>) -> Self {
        Self::UnsupportedVersion {
            version: version.into(),
        }
    }
}
