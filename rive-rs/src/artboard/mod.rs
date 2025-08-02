use alloc::sync::Arc;
use core::{fmt, marker::PhantomData, ptr::NonNull};

use crate::{
    ffi,
    file::{File, FileInner},
    instantiate::{Handle, Instantiate},
    renderer::Renderer,
};

use self::components::Components;

pub mod components;

#[derive(Debug)]
pub(crate) struct ArtboardInner {
    _file: Arc<FileInner>,
    pub(crate) raw_artboard: *mut ffi::Artboard,
}

impl Drop for ArtboardInner {
    fn drop(&mut self) {
        unsafe {
            ffi::rive_rs_artboard_instance_release(self.raw_artboard);
        }
    }
}

unsafe impl Send for ArtboardInner {}
unsafe impl Sync for ArtboardInner {}

pub struct Artboard<R: Renderer> {
    inner: Arc<ArtboardInner>,
    _phantom: PhantomData<R>,
}

impl<R: Renderer> Artboard<R> {
    pub(crate) fn from_inner(inner: Arc<ArtboardInner>) -> Self {
        Self {
            inner,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn as_inner(&self) -> &Arc<ArtboardInner> {
        &self.inner
    }

    #[inline]
    pub fn components(&mut self) -> Components {
        Components::new(components::RawArtboard(self.inner.raw_artboard))
    }

    // Artboard property getters and setters
    
    /// Gets the width of the artboard
    pub fn width(&self) -> f32 {
        unsafe {
            ffi::rive_rs_artboard_get_width(self.inner.raw_artboard)
        }
    }

    /// Sets the width of the artboard
    pub fn set_width(&mut self, width: f32) {
        unsafe {
            ffi::rive_rs_artboard_set_width(self.inner.raw_artboard, width);
        }
    }

    /// Gets the height of the artboard
    pub fn height(&self) -> f32 {
        unsafe {
            ffi::rive_rs_artboard_get_height(self.inner.raw_artboard)
        }
    }

    /// Sets the height of the artboard
    pub fn set_height(&mut self, height: f32) {
        unsafe {
            ffi::rive_rs_artboard_set_height(self.inner.raw_artboard, height);
        }
    }

    /// Sets both width and height of the artboard
    pub fn set_dimensions(&mut self, width: f32, height: f32) {
        self.set_width(width);
        self.set_height(height);
    }

    /// Gets the origin X coordinate of the artboard
    pub fn origin_x(&self) -> f32 {
        unsafe {
            ffi::rive_rs_artboard_get_origin_x(self.inner.raw_artboard)
        }
    }

    /// Sets the origin X coordinate of the artboard
    pub fn set_origin_x(&mut self, origin_x: f32) {
        unsafe {
            ffi::rive_rs_artboard_set_origin_x(self.inner.raw_artboard, origin_x);
        }
    }

    /// Gets the origin Y coordinate of the artboard
    pub fn origin_y(&self) -> f32 {
        unsafe {
            ffi::rive_rs_artboard_get_origin_y(self.inner.raw_artboard)
        }
    }

    /// Sets the origin Y coordinate of the artboard
    pub fn set_origin_y(&mut self, origin_y: f32) {
        unsafe {
            ffi::rive_rs_artboard_set_origin_y(self.inner.raw_artboard, origin_y);
        }
    }

    /// Sets both origin X and Y coordinates of the artboard
    pub fn set_origin(&mut self, origin_x: f32, origin_y: f32) {
        self.set_origin_x(origin_x);
        self.set_origin_y(origin_y);
    }

    /// Gets whether the artboard clips its contents
    pub fn clip(&self) -> bool {
        unsafe {
            ffi::rive_rs_artboard_get_clip(self.inner.raw_artboard)
        }
    }

    /// Sets whether the artboard clips its contents
    pub fn set_clip(&mut self, clip: bool) {
        unsafe {
            ffi::rive_rs_artboard_set_clip(self.inner.raw_artboard, clip);
        }
    }
}

impl<R: Renderer> Instantiate for Artboard<R> {
    type From = File<R>;

    #[inline]
    fn instantiate(file: &Self::From, handle: Handle) -> Option<Self> {
        let mut raw_artboard: Option<NonNull<ffi::Artboard>> = None;

        match handle {
            Handle::Default => unsafe {
                ffi::rive_rs_instantiate_artboard(file.as_inner().raw_file, None, &mut raw_artboard)
            },
            Handle::Index(ref index) => unsafe {
                ffi::rive_rs_instantiate_artboard(
                    file.as_inner().raw_file,
                    Some(index.into()),
                    &mut raw_artboard,
                )
            },
            Handle::Name(name) => unsafe {
                ffi::rive_rs_instantiate_artboard_by_name(
                    file.as_inner().raw_file,
                    name.as_ptr(),
                    name.len(),
                    &mut raw_artboard,
                )
            },
        }

        raw_artboard.map(|raw_artboard| Artboard {
            inner: Arc::new(ArtboardInner {
                _file: file.as_inner().clone(),
                raw_artboard: raw_artboard.as_ptr(),
            }),
            _phantom: PhantomData,
        })
    }
}

impl<R: Renderer> fmt::Debug for Artboard<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ardboard").finish()
    }
}

unsafe impl<R: Renderer> Send for Artboard<R> {}
unsafe impl<R: Renderer> Sync for Artboard<R> {}
