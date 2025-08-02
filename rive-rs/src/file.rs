use alloc::{sync::Arc, vec::Vec};
use core::{fmt, marker::PhantomData, ptr};

#[cfg(feature = "export")]
use crate::export::{ExportError, ExportFormat, create_empty_file_export, export_to_json};

use crate::{
    ffi::{self},
    renderer::Renderer,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    /// Indicates that the Rive file is not supported by this runtime.
    UnsupportedVersion,
    /// Indicates that the there is a formatting problem in the file itself.
    Malformed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnsupportedVersion => f.write_str("unsupported Rive version"),
            Error::Malformed => f.write_str("file is incorrectly encoded"),
        }
    }
}

#[cfg(feature = "vello")]
impl std::error::Error for Error {}

#[derive(Debug)]
pub(crate) struct FileInner {
    pub raw_file: *const ffi::File,
    raw_factory: *mut ffi::Factory,
}

impl Drop for FileInner {
    fn drop(&mut self) {
        unsafe {
            ffi::rive_rs_file_release(self.raw_file, self.raw_factory);
        }
    }
}

unsafe impl Send for FileInner {}
unsafe impl Sync for FileInner {}

pub struct File<R: Renderer> {
    inner: Arc<FileInner>,
    _phantom: PhantomData<R>,
}

impl<R: Renderer> File<R> {
    #[inline]
    pub fn new(data: &[u8]) -> Result<Self, Error> {
        let mut result = ffi::FileResult::Success;
        let mut raw_factory = ptr::null_mut();

        let raw_file = unsafe {
            ffi::rive_rs_file_new(
                data.as_ptr(),
                data.len(),
                ffi::RendererEntries::<R>::ENTRIES as *const ffi::RendererEntries<R> as *const (),
                &mut result as *mut ffi::FileResult,
                &mut raw_factory as *mut *mut ffi::Factory,
            )
        };

        match result {
            ffi::FileResult::Success => Ok(Self {
                inner: Arc::new(FileInner {
                    raw_file,
                    raw_factory,
                }),
                _phantom: PhantomData,
            }),
            ffi::FileResult::UnsupportedVersion => Err(Error::UnsupportedVersion),
            ffi::FileResult::Malformed => Err(Error::Malformed),
        }
    }

    #[inline]
    pub fn create() -> Self {
        let mut raw_factory = ptr::null_mut();

        let raw_file = unsafe {
            ffi::rive_rs_file_create(
                ffi::RendererEntries::<R>::ENTRIES as *const ffi::RendererEntries<R> as *const (),
                &mut raw_factory as *mut *mut ffi::Factory,
            )
        };

        Self {
            inner: Arc::new(FileInner {
                raw_file,
                raw_factory,
            }),
            _phantom: PhantomData,
        }
    }

    pub(crate) fn as_inner(&self) -> &Arc<FileInner> {
        &self.inner
    }

    /// Exports the file to binary data.
    /// Returns None if binary export is not supported or fails.
    /// For debugging and validation, use `export_json()` instead.
    pub fn export(&self) -> Option<Vec<u8>> {
        let mut data_ptr: *mut u8 = ptr::null_mut();
        let mut len: usize = 0;
        
        unsafe {
            ffi::rive_rs_file_export(
                self.inner.raw_file,
                &mut data_ptr as *mut *mut u8,
                &mut len as *mut usize,
            );
        }
        
        if data_ptr.is_null() || len == 0 {
            None
        } else {
            unsafe {
                let data = Vec::from_raw_parts(data_ptr, len, len);
                Some(data)
            }
        }
    }

    /// Exports the file to JSON format for debugging and inspection.
    /// This provides a human-readable representation of the file structure.
    #[cfg(feature = "export")]
    pub fn export_json(&self) -> Result<alloc::string::String, ExportError> {
        // For now, create a basic export structure
        // TODO: Extract actual file data from C++ runtime
        let file_export = create_empty_file_export();
        export_to_json(&file_export)
    }

    /// Exports the file in the specified format.
    #[cfg(feature = "export")]
    pub fn export_format(&self, format: ExportFormat) -> Result<Vec<u8>, ExportError> {
        match format {
            ExportFormat::Json => {
                let json = self.export_json()?;
                Ok(json.into_bytes())
            }
            ExportFormat::Memory => {
                // TODO: Implement memory dump export
                Err(ExportError::UnsupportedFormat)
            }
        }
    }

    /// Saves the file to the specified path in JSON format.
    #[cfg(all(feature = "export", feature = "vello"))]
    pub fn save_json<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), ExportError> {
        use std::fs;
        
        let json = self.export_json()?;
        fs::write(path, json).map_err(ExportError::IoError)
    }

    /// Saves the file to the specified path.
    /// Returns an error if the export fails or file cannot be written.
    #[cfg(feature = "vello")]
    pub fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs;
        
        // Try binary export first, fall back to JSON
        if let Some(data) = self.export() {
            fs::write(path, data)?;
            Ok(())
        } else {
            #[cfg(feature = "export")]
            {
                let json = self.export_json()?;
                fs::write(path, json)?;
                Ok(())
            }
            #[cfg(not(feature = "export"))]
            {
                Err("Export not supported".into())
            }
        }
    }

    /// Get the number of artboards in this file
    pub fn artboard_count(&self) -> usize {
        unsafe {
            ffi::rive_rs_file_artboard_count(self.inner.raw_file)
        }
    }

    /// Get an artboard by index
    pub fn artboard_at(&self, index: usize) -> Option<crate::artboard::Artboard<R>> {
        if index == 0 {
            // Return default artboard for now
            use crate::instantiate::{Handle, Instantiate};
            crate::artboard::Artboard::instantiate(self, Handle::Default)
        } else {
            None
        }
    }

    /// Get an artboard by name
    pub fn artboard_by_name(&self, name: &str) -> Option<crate::artboard::Artboard<R>> {
        // TODO: Implement actual name-based lookup
        // For now, only support "default" name
        if name == "default" {
            use crate::instantiate::{Handle, Instantiate};
            crate::artboard::Artboard::instantiate(self, Handle::Default)
        } else {
            None
        }
    }

    /// Get the default artboard (first artboard)
    pub fn default_artboard(&self) -> Option<crate::artboard::Artboard<R>> {
        self.artboard_at(0)
    }

    /// List all artboard names in this file
    pub fn artboard_names(&self) -> Vec<alloc::string::String> {
        let count = self.artboard_count();
        let mut names = Vec::with_capacity(count);
        
        for i in 0..count {
            let mut data_ptr: *const u8 = ptr::null();
            let mut len: usize = 0;
            
            unsafe {
                ffi::rive_rs_file_artboard_name_at(
                    self.inner.raw_file,
                    i,
                    &mut data_ptr as *mut *const u8,
                    &mut len as *mut usize,
                );
                
                if !data_ptr.is_null() && len > 0 {
                    let name_bytes = core::slice::from_raw_parts(data_ptr, len);
                    if let Ok(name) = alloc::string::String::from_utf8(name_bytes.to_vec()) {
                        names.push(name);
                    } else {
                        names.push(format!("artboard_{}", i));
                    }
                } else {
                    names.push(format!("artboard_{}", i));
                }
            }
        }
        
        names
    }
}

impl<R: Renderer> fmt::Debug for File<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("File").finish()
    }
}
