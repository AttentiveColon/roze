use crate::input::Input;
use crate::renderer::Renderer;
use crate::system::win32::*;
use std::alloc;

#[derive(Debug)]
pub struct WindowResources {
    pub width: i32,
    pub height: i32,
    pub input: *mut Input,
    pub renderer: *mut Renderer,
    pub running: bool,
    pub show_cursor: bool,
    pub hide_cursor: bool,
}

impl WindowResources {
    pub const unsafe fn const_new() -> Self {
        WindowResources {
            width: 0,
            height: 0,
            input: std::ptr::null_mut(),
            renderer: std::ptr::null_mut(),
            running: true,
            show_cursor: false,
            hide_cursor: false,
        }
    }
}

#[derive(Debug)]
pub struct WindowDimension {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug)]
pub struct Window {
    pub window_handle: HWND,
    pub device_context: HDC,
    pub dimensions: WindowDimension,
    pub window_resources: *mut WindowResources,
}

impl Window {
    pub fn create_window(window_title: &str, width: i32, height: i32) -> Window {
        unsafe {
            let instance = GetModuleHandleA(std::ptr::null());
            debug_assert!(instance != 0);

            let cursor = LoadCursorW(instance, IDC_ARROW);

            let window_class_name = "window\0".as_ptr();
            let wc = WNDCLASSA {
                hCursor: cursor, //LoadCursorW(instance, IDC_ARROW),
                hInstance: instance,
                lpszClassName: window_class_name,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: wndproc,
                hIcon: std::mem::zeroed(),
                hbrBackground: std::mem::zeroed(),
                lpszMenuName: std::mem::zeroed(),
                cbWndExtra: std::mem::zeroed(),
                cbClsExtra: std::mem::zeroed(),
            };

            let atom = RegisterClassA(&wc);
            debug_assert!(atom != 0);

            let mut window_title = String::from(window_title);
            window_title.push('\0');

            let window_handle = CreateWindowExA(
                WINDOW_EX_STYLE::default(),
                window_class_name,
                window_title.as_ptr(),
                WS_SYSMENU | WS_MINIMIZEBOX | WS_VISIBLE | CS_OWNDC,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width,
                height,
                std::mem::zeroed(),
                std::mem::zeroed(),
                instance,
                std::ptr::null(),
            );
            debug_assert!(window_handle != 0);

            let device_context = GetDC(window_handle);

            let window_resources =
                GetWindowLongPtrA(window_handle, GWLP_USERDATA) as *mut WindowResources;
            let dimensions = get_window_dimensions(window_handle);

            Window {
                window_handle,
                device_context,
                dimensions,
                window_resources,
            }
        }
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        let resources = GetWindowLongPtrA(window, GWLP_USERDATA) as *mut WindowResources;

        match message {
            WM_CREATE => {
                let resource_layout = alloc::Layout::new::<WindowResources>();
                let ptr = alloc::alloc(resource_layout);
                let ptr = std::mem::transmute::<*mut u8, *mut WindowResources>(ptr);
                *ptr = WindowResources::const_new();
                SetWindowLongPtrA(window, GWLP_USERDATA, ptr as isize);

                let client_rect = RECT::new();
                GetClientRect(window, &client_rect);
                let window_rect = RECT::new();
                GetWindowRect(window, &window_rect);

                let width_diff =
                    (window_rect.right - window_rect.left) - (client_rect.right - client_rect.left);
                let height_diff =
                    (window_rect.bottom - window_rect.top) - (client_rect.bottom - client_rect.top);

                let rect = RECT {
                    left: window_rect.left,
                    top: window_rect.top,
                    right: window_rect.right + width_diff,
                    bottom: window_rect.bottom + height_diff,
                };

                SetWindowPos(
                    window,
                    0 as HWND,
                    rect.left,
                    rect.top,
                    rect.right - rect.left,
                    rect.bottom - rect.top,
                    2,
                );

                0
            }
            WM_LBUTTONDOWN | WM_LBUTTONUP | WM_RBUTTONDOWN | WM_RBUTTONUP | WM_MBUTTONDOWN
            | WM_MBUTTONUP | WM_XBUTTONDOWN | WM_XBUTTONUP | WM_MOUSEHWHEEL | WM_MOUSEWHEEL
            | WM_SYSKEYDOWN | WM_SYSKEYUP | WM_KEYDOWN | WM_KEYUP => {
                (*(*resources).input).process_messages(message, wparam)
            }
            WM_CLOSE => {
                (*resources).running = false;
                PostQuitMessage(0);
                0
            }
            WM_ACTIVATEAPP => 0,
            WM_DESTROY => {
                (*resources).running = false;
                PostQuitMessage(0);
                0
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}

pub fn _failed(value: i32) {
    debug_assert!(value >= 0);
}

pub fn get_window_dimensions(hwnd: HWND) -> WindowDimension {
    unsafe {
        let rect = RECT::new();
        GetClientRect(hwnd, &rect);
        WindowDimension {
            width: rect.right - rect.left,
            height: rect.bottom - rect.top,
        }
    }
}
