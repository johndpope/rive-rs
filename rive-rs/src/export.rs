//! Export functionality for Rive files
//! 
//! This module provides various export formats for Rive files, enabling
//! debugging, validation, and workflow integration.

#[cfg(feature = "export")]
use alloc::{string::{String, ToString}, vec::Vec};

#[cfg(feature = "export")]
use serde::{Deserialize, Serialize};

/// Error types for export operations
#[derive(Debug)]
pub enum ExportError {
    /// Export format not supported
    UnsupportedFormat,
    /// Serialization failed
    SerializationFailed(String),
    /// File system error
    #[cfg(feature = "vello")]
    IoError(std::io::Error),
}

impl core::fmt::Display for ExportError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ExportError::UnsupportedFormat => write!(f, "Export format not supported"),
            ExportError::SerializationFailed(msg) => write!(f, "Serialization failed: {}", msg),
            #[cfg(feature = "vello")]
            ExportError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

#[cfg(feature = "vello")]
impl std::error::Error for ExportError {}

/// Represents the structure of a Rive file for export
#[cfg(feature = "export")]
#[derive(Debug, Serialize, Deserialize)]
pub struct FileExport {
    /// File format version
    pub version: String,
    /// File metadata
    pub metadata: FileMetadata,
    /// Artboards in the file
    pub artboards: Vec<ArtboardExport>,
    /// Assets in the file
    pub assets: Vec<AssetExport>,
}

/// Metadata about the exported file
#[cfg(feature = "export")]
#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    /// When the file was exported
    pub exported_at: String,
    /// Export format version
    pub export_format_version: String,
    /// Original file source
    pub source: String,
}

/// Represents an artboard for export
#[cfg(feature = "export")]
#[derive(Debug, Serialize, Deserialize)]
pub struct ArtboardExport {
    /// Artboard name
    pub name: String,
    /// Artboard dimensions
    pub width: f32,
    /// Artboard height
    pub height: f32,
    /// Components in the artboard
    pub components: Vec<ComponentExport>,
}

/// Represents a component for export
#[cfg(feature = "export")]
#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentExport {
    /// Component type
    pub component_type: String,
    /// Component name
    pub name: String,
    /// Component properties
    pub properties: serde_json::Value,
}

/// Represents an asset for export
#[cfg(feature = "export")]
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetExport {
    /// Asset name
    pub name: String,
    /// Asset type
    pub asset_type: String,
    /// Asset metadata
    pub metadata: serde_json::Value,
}

/// Export formats supported by rive-rs
#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    /// JSON format for debugging and inspection
    Json,
    /// Raw memory dump for analysis
    Memory,
}

/// Export a file to JSON format
#[cfg(feature = "export")]
pub fn export_to_json(file_export: &FileExport) -> Result<String, ExportError> {
    serde_json::to_string_pretty(file_export)
        .map_err(|e| ExportError::SerializationFailed(e.to_string()))
}

/// Create a basic file export structure for an empty file
#[cfg(feature = "export")]
pub fn create_empty_file_export() -> FileExport {
    FileExport {
        version: "1.0".to_string(),
        metadata: FileMetadata {
            exported_at: "2025-08-02T00:00:00Z".to_string(), // TODO: Use actual timestamp
            export_format_version: "1.0".to_string(),
            source: "rive-rs::File::create()".to_string(),
        },
        artboards: Vec::new(),
        assets: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "export")]
    fn test_empty_file_export() {
        let export = create_empty_file_export();
        assert_eq!(export.version, "1.0");
        assert_eq!(export.artboards.len(), 0);
        assert_eq!(export.assets.len(), 0);
    }

    #[test]
    #[cfg(feature = "export")]
    fn test_json_export() {
        let export = create_empty_file_export();
        let json = export_to_json(&export).unwrap();
        assert!(json.contains("\"version\": \"1.0\""));
        assert!(json.contains("\"artboards\": []"));
    }
}