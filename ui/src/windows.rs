//! Functions and types related to windows.

use controls::Control;
use ffi_utils::{self, Text};
use libc::{c_int, c_void};
use std::cell::RefCell;
use std::ffi::CString;
use std::mem;
use ui_sys::{self, uiControl, uiWindow};

thread_local! {
    static WINDOWS: RefCell<Vec<Window>> = RefCell::new(Vec::new())
}

define_control!(Window, uiWindow, ui_window);

impl Window {
    #[inline]
    pub fn as_ui_window(&self) -> *mut uiWindow {
        self.ui_window
    }

    #[inline]
    pub fn title(&self) -> Text {
        ffi_utils::ensure_initialized();
        unsafe {
            Text::new(ui_sys::uiWindowTitle(self.ui_window))
        }
    }

    #[inline]
    pub fn set_title(&self, title: &str) {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(title.as_bytes().to_vec()).unwrap();
            ui_sys::uiWindowSetTitle(self.ui_window, c_string.as_ptr())
        }
    }

    #[inline]
    pub fn position(&self) -> (i32, i32) {
        ffi_utils::ensure_initialized();
        let mut x: c_int = 0;
        let mut y: c_int = 0;
        unsafe {
            ui_sys::uiWindowPosition(self.ui_window, &mut x, &mut y);
        }
        (x, y)
    }

    #[inline]
    pub fn set_position(&self, x: i32, y: i32) {
        ffi_utils::ensure_initialized();
        unsafe {
            ui_sys::uiWindowSetPosition(self.ui_window, x as c_int, y as c_int)
        }
    }

    #[inline]
    pub fn center(&self) {
        ffi_utils::ensure_initialized();
        unsafe {
            ui_sys::uiWindowCenter(self.ui_window)
        }
    }

    #[inline]
    pub fn on_position_changed(&self, callback: Box<FnMut(&Window)>) {
        ffi_utils::ensure_initialized();
        unsafe {
            let mut data: Box<Box<FnMut(&Window)>> = Box::new(callback);
            ui_sys::uiWindowOnPositionChanged(self.ui_window,
                                              c_callback,
                                              &mut *data as *mut Box<FnMut(&Window)> as
                                              *mut c_void);
            mem::forget(data);
        }

        extern "C" fn c_callback(window: *mut uiWindow, data: *mut c_void) {
            unsafe {
                let window = Window {
                    ui_window: window,
                };
                mem::transmute::<*mut c_void, &mut Box<FnMut(&Window)>>(data)(&window)
            }
        }
    }

    #[inline]
    pub fn content_size(&self) -> (i32, i32) {
        ffi_utils::ensure_initialized();
        let mut width: c_int = 0;
        let mut height: c_int = 0;
        unsafe {
            ui_sys::uiWindowContentSize(self.ui_window, &mut width, &mut height);
        }
        (width, height)
    }

    #[inline]
    pub fn set_content_size(&self, width: i32, height: i32) {
        ffi_utils::ensure_initialized();
        unsafe {
            ui_sys::uiWindowSetContentSize(self.ui_window, width as c_int, height as c_int)
        }
    }

    #[inline]
    pub fn fullscreen(&self) -> bool {
        ffi_utils::ensure_initialized();
        unsafe {
            ui_sys::uiWindowFullscreen(self.ui_window) != 0
        }
    }

    #[inline]
    pub fn set_fullscreen(&self, fullscreen: bool) {
        ffi_utils::ensure_initialized();
        unsafe {
            ui_sys::uiWindowSetFullscreen(self.ui_window, fullscreen as c_int)
        }
    }

    #[inline]
    pub fn on_content_size_changed(&self, callback: Box<FnMut(&Window)>) {
        ffi_utils::ensure_initialized();
        unsafe {
            let mut data: Box<Box<FnMut(&Window)>> = Box::new(callback);
            ui_sys::uiWindowOnContentSizeChanged(self.ui_window,
                                                 c_callback,
                                                 &mut *data as *mut Box<FnMut(&Window)> as
                                                 *mut c_void);
            mem::forget(data);
        }

        extern "C" fn c_callback(window: *mut uiWindow, data: *mut c_void) {
            unsafe {
                let window = Window {
                    ui_window: window,
                };
                mem::transmute::<*mut c_void, &mut Box<FnMut(&Window)>>(data)(&window)
            }
        }
    }

    #[inline]
    pub fn on_closing(&self, callback: Box<FnMut(&Window) -> bool>) {
        ffi_utils::ensure_initialized();
        unsafe {
            let mut data: Box<Box<FnMut(&Window) -> bool>> = Box::new(callback);
            ui_sys::uiWindowOnClosing(self.ui_window,
                                      c_callback,
                                      &mut *data as *mut Box<FnMut(&Window) -> bool> as
                                      *mut c_void);
            mem::forget(data);
        }

        extern "C" fn c_callback(window: *mut uiWindow, data: *mut c_void) -> i32 {
            unsafe {
                let window = Window {
                    ui_window: window,
                };
                mem::transmute::<*mut c_void,
                                 Box<Box<FnMut(&Window) -> bool>>>(data)(&window) as i32
            }
        }
    }

    #[inline]
    pub fn borderless(&self) -> bool {
        ffi_utils::ensure_initialized();
        unsafe {
            ui_sys::uiWindowBorderless(self.ui_window) != 0
        }
    }

    #[inline]
    pub fn set_borderless(&self, borderless: bool) {
        ffi_utils::ensure_initialized();
        unsafe {
            ui_sys::uiWindowSetBorderless(self.ui_window, borderless as c_int)
        }
    }

    #[inline]
    pub fn set_child(&self, child: Control) {
        ffi_utils::ensure_initialized();
        unsafe {
            ui_sys::uiWindowSetChild(self.ui_window, child.as_ui_control())
        }
    }

    #[inline]
    pub fn margined(&self) -> bool {
        ffi_utils::ensure_initialized();
        unsafe {
            ui_sys::uiWindowMargined(self.ui_window) != 0
        }
    }

    #[inline]
    pub fn set_margined(&self, margined: bool) {
        ffi_utils::ensure_initialized();
        unsafe {
            ui_sys::uiWindowSetMargined(self.ui_window, margined as c_int)
        }
    }

    #[inline]
    pub fn new(title: &str, width: c_int, height: c_int, has_menubar: bool) -> Window {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(title.as_bytes().to_vec()).unwrap();
            let window = Window::from_ui_window(ui_sys::uiNewWindow(c_string.as_ptr(),
                                                                    width,
                                                                    height,
                                                                    has_menubar as c_int));

            WINDOWS.with(|windows| windows.borrow_mut().push(window.clone()));

            window
        }
    }

    #[inline]
    pub unsafe fn from_ui_window(window: *mut uiWindow) -> Window {
        Window {
            ui_window: window,
        }
    }

    pub unsafe fn destroy_all_windows() {
        WINDOWS.with(|windows| {
            let mut windows = windows.borrow_mut();
            for window in windows.drain(..) {
                window.destroy()
            }
        })
    }
}
