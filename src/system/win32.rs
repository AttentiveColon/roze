#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::style)]

use std::ffi::c_void;

///////////////////////////////////////////////////////////////////////////////////////////////////
// WIN32 TYPES
///////////////////////////////////////////////////////////////////////////////////////////////////

pub type HINSTANCE = isize;
pub type HWND = isize;
pub type HDC = isize;
pub type HGDIOBJ = *const c_void;
pub type HBITMAP = isize;
pub type HMENU = isize;
pub type HCURSOR = isize;
pub type HBRUSH = isize;
pub type HICON = isize;
pub type HANDLE = isize;
pub type DWORD = u32;
pub type LPARAM = isize;
pub type LRESULT = isize;
pub type WPARAM = usize;
pub type WINDOW_STYLE = u32;
pub type WNDCLASS_STYLES = u32;
pub type MESSAGEBOX_STYLE = u32;
pub type WINDOW_EX_STYLE = u32;
pub type DIB_USAGE = u32;
pub type ROP_CODE = u32;
pub type BOOL = i32;
pub type MESSAGEBOX_RESULT = i32;
pub type BYTE = u8;
pub type PCSTR = *const u8;
pub type PCWSTR = *const u16;
pub type LPVOID = *mut c_void;
pub type PVOID = *mut c_void;
pub type WORD = u16;
pub type SHORT = i16;
pub type UINT = u32;
pub type LONG_PTR = isize;
pub type VKCODE = u32;
pub type LONG = i32;
pub type PLONG = *mut LONG;
pub type LPDWORD = *mut DWORD;
pub type LPOVERLAPPED = *mut c_void;
pub type LPSECURITY_ATTRIBUTES = *mut c_void;
pub type HRESULT = LONG;
pub type COINIT = u32;
pub type UINT32 = u32;
pub type LPCWSTR = *const u16;
pub type LPCSTR = *const u8;
pub type FLOAT = f32;
pub type ULONG = u32;
pub type PULONG = *mut ULONG;
pub type HFONT = *const c_void;

///////////////////////////////////////////////////////////////////////////////////////////////////
// WIN32 CONSTANTS
///////////////////////////////////////////////////////////////////////////////////////////////////

pub const CS_HREDRAW: WNDCLASS_STYLES = 2;
pub const CS_VREDRAW: WNDCLASS_STYLES = 1;
pub const CS_OWNDC: WNDCLASS_STYLES = 32;

pub const WM_CREATE: u32 = 1u32;
pub const WM_ACTIVATEAPP: u32 = 28u32;
pub const WM_PAINT: u32 = 15u32;
pub const WM_SIZE: u32 = 5u32;
pub const WM_CLOSE: u32 = 16u32;
pub const WM_DESTROY: u32 = 2u32;
pub const WM_QUIT: u32 = 18u32;
pub const WM_SYSKEYDOWN: u32 = 260u32;
pub const WM_SYSKEYUP: u32 = 261u32;
pub const WM_KEYDOWN: u32 = 256u32;
pub const WM_KEYUP: u32 = 257u32;
pub const WM_SETCURSOR: u32 = 0x20u32;

pub const WM_LBUTTONDOWN: u32 = 513u32;
pub const WM_LBUTTONUP: u32 = 514u32;
pub const WM_RBUTTONDOWN: u32 = 516;
pub const WM_RBUTTONUP: u32 = 517;
pub const WM_MBUTTONDOWN: u32 = 519;
pub const WM_MBUTTONUP: u32 = 520;
pub const WM_XBUTTONDOWN: u32 = 523;
pub const WM_XBUTTONUP: u32 = 524;

pub const WM_MOUSEWHEEL: u32 = 522;
pub const WM_MOUSEHWHEEL: u32 = 526;

pub const WS_OVERLAPPEDWINDOW: WINDOW_STYLE = 13565952;
pub const WS_VISIBLE: WINDOW_STYLE = 268435456;

pub const WS_MINIMIZEBOX: WINDOW_STYLE = 0x00020000;
pub const WS_MAXIMIZEBOX: WINDOW_STYLE = 0x00010000;
pub const WS_CAPTION: WINDOW_STYLE = 0x00C0000;
pub const WS_SYSMENU: WINDOW_STYLE = 0x00080000;
pub const WS_THICKFRAME: WINDOW_STYLE = 0x00040000;
pub const WS_BORDER: WINDOW_STYLE = 0x00800000;

pub const IDC_ARROW: PCWSTR = 32512i32 as _;

pub const DIB_RGB_COLORS: DIB_USAGE = 0u32;

pub const SRCCOPY: ROP_CODE = 13369376u32;

pub const CW_USEDEFAULT: i32 = -2147483648i32;
pub const BI_RGB: u32 = 0u32;

pub const MEM_COMMIT: DWORD = 0x1000;
pub const MEM_RESERVE: DWORD = 0x2000;
pub const MEM_RELEASE: DWORD = 0x8000;

pub const PAGE_READWRITE: DWORD = 0x04;

pub const XUSER_MAX_COUNT: DWORD = 4;

pub const XINPUT_GAMEPAD_DPAD_UP: WORD = 0x0001;
pub const XINPUT_GAMEPAD_DPAD_DOWN: WORD = 0x0002;
pub const XINPUT_GAMEPAD_DPAD_LEFT: WORD = 0x0004;
pub const XINPUT_GAMEPAD_DPAD_RIGHT: WORD = 0x0008;
pub const XINPUT_GAMEPAD_START: WORD = 0x0010;
pub const XINPUT_GAMEPAD_BACK: WORD = 0x0020;
pub const XINPUT_GAMEPAD_LEFT_THUMB: WORD = 0x0040;
pub const XINPUT_GAMEPAD_RIGHT_THUMB: WORD = 0x0080;
pub const XINPUT_GAMEPAD_LEFT_SHOULDER: WORD = 0x0100;
pub const XINPUT_GAMEPAD_RIGHT_SHOULDER: WORD = 0x0200;
pub const XINPUT_GAMEPAD_A: WORD = 0x1000;
pub const XINPUT_GAMEPAD_B: WORD = 0x2000;
pub const XINPUT_GAMEPAD_X: WORD = 0x4000;
pub const XINPUT_GAMEPAD_Y: WORD = 0x8000;

pub const ERROR_SUCCESS: DWORD = 0;

pub const PM_NOREMOVE: UINT = 0x0000;
pub const PM_REMOVE: UINT = 0x0001;
pub const PM_NOYIELD: UINT = 0x0002;

pub const GENERIC_READ: u32 = 0x80000000;
pub const GENERIC_WRITE: u32 = 0x40000000;
pub const GENERIC_EXECUTE: u32 = 0x20000000;
pub const GENERIC_ALL: u32 = 0x10000000;

pub const GWLP_USERDATA: i32 = -21;

pub const VK_LBUTTON: u32 = 1;
pub const VK_RBUTTON: u32 = 2;

pub const VK_MBUTTON: u32 = 4;
pub const VK_XBUTTON1: u32 = 5;
pub const VK_XBUTTON2: u32 = 6;

pub const VK_ESCAPE: u32 = 27;

pub const FILE_SHARE_READ: u32 = 0x1;

pub const OPEN_EXISTING: u32 = 3;
pub const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;

pub const INVALID_HANDLE_VALUE: isize = -1;
pub const INVALID_SET_FILE_POINTER: DWORD = std::u32::MAX;

pub const FILE_BEGIN: u32 = 0;

pub const FW_DONTCARE: i32 = 0;
pub const FW_THIN: i32 = 100;
pub const FW_EXTRALIGHT: i32 = 200;
pub const FW_ULTRALIGHT: i32 = 200;
pub const FW_LIGHT: i32 = 300;
pub const FW_NORMAL: i32 = 400;
pub const FW_REGULAR: i32 = 400;
pub const FW_MEDIUM: i32 = 500;
pub const FW_SEMIBOLD: i32 = 600;
pub const FW_DEMIBOLD: i32 = 600;
pub const FW_BOLD: i32 = 700;
pub const FW_EXTRABOLD: i32 = 800;
pub const FW_ULTRABOLD: i32 = 800;
pub const FW_HEAVY: i32 = 900;
pub const FW_BLACK: i32 = 900;

pub const DEFAULT_CHARSET: u8 = 1;

pub const OUT_OUTLINE_PRECIS: u8 = 8;

pub const CLIP_DEFAULT_PRECIS: u8 = 0;

pub const CLEARTYPE_QUALITY: u8 = 5;

pub const VARIABLE_PITCH: u8 = 2;

///////////////////////////////////////////////////////////////////////////////////////////////////
// WIN32 STRUCTURES
///////////////////////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct WNDCLASSA {
    pub style: WNDCLASS_STYLES,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON, //<==
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH, //<==
    pub lpszMenuName: PCSTR,
    pub lpszClassName: PCSTR,
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct BITMAPINFO {
    pub bmiHeader: BITMAPINFOHEADER,
    pub bmiColors: [RGBQUAD; 1],
}

impl BITMAPINFO {
    pub const fn const_default() -> Self {
        BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER::const_default(),
            bmiColors: [RGBQUAD::const_default()],
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct BITMAPINFOHEADER {
    pub biSize: u32,
    pub biWidth: i32,
    pub biHeight: i32,
    pub biPlanes: u16,
    pub biBitCount: u16,
    pub biCompression: u32,
    pub biSizeImage: u32,
    pub biXPelsPerMeter: i32,
    pub biYPelsPerMeter: i32,
    pub biClrUsed: u32,
    pub biClrImportant: u32,
}

impl BITMAPINFOHEADER {
    pub const fn const_default() -> Self {
        BITMAPINFOHEADER {
            biSize: 0,
            biWidth: 0,
            biHeight: 0,
            biPlanes: 0,
            biBitCount: 0,
            biCompression: 0,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct RGBQUAD {
    pub rgbBlue: u8,
    pub rgbGreen: u8,
    pub rgbRed: u8,
    pub rgbReserved: u8,
}

impl RGBQUAD {
    pub const fn const_default() -> Self {
        RGBQUAD {
            rgbBlue: 0,
            rgbGreen: 0,
            rgbRed: 0,
            rgbReserved: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl RECT {
    pub fn new() -> Self {
        RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }
    pub const fn as_null_ptr() -> *const Self {
        std::ptr::null()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PAINTSTRUCT {
    pub hdc: HDC,
    pub fErase: BOOL,
    pub rcPaint: RECT,
    fRestore: BOOL,
    fIncUpdate: BOOL,
    rgbReserved: [BYTE; 32],
}

impl PAINTSTRUCT {
    pub fn new() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: u32,
    pub pt: POINT,
}

impl Default for MSG {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}

impl Default for POINT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct XINPUT_STATE {
    pub dwPacketNumber: DWORD,
    pub Gamepad: XINPUT_GAMEPAD,
}

impl Default for XINPUT_STATE {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct XINPUT_VIBRATION {
    pub wLeftMotorSpeed: WORD,
    pub wRightMotorSpeed: WORD,
}

impl XINPUT_VIBRATION {
    pub fn new(vibration_level: u16) -> Self {
        XINPUT_VIBRATION {
            wLeftMotorSpeed: vibration_level,
            wRightMotorSpeed: vibration_level,
        }
    }

    pub fn set(&mut self, vibration_level: u16) {
        self.wLeftMotorSpeed = vibration_level;
        self.wRightMotorSpeed = vibration_level;
    }
}

impl Default for XINPUT_VIBRATION {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct XINPUT_GAMEPAD {
    pub wButtons: WORD,
    pub bLeftTrigger: BYTE,
    pub bRightTrigger: BYTE,
    pub sThumbLX: SHORT,
    pub sThumbLY: SHORT,
    pub sThumbRX: SHORT,
    pub sThumbRY: SHORT,
}

#[repr(C)]
pub struct LOGFONTA {
    pub lfHeight: LONG,
    pub lfWidth: LONG,
    pub lfEscapement: LONG,
    pub lfOrientation: LONG,
    pub lfWeigth: LONG,
    pub lfItalic: BYTE,
    pub lfUnderline: BYTE,
    pub lfStrikeOut: BYTE,
    pub lfCharSet: BYTE,
    pub lfOutPrecision: BYTE,
    pub lfClipPrecision: BYTE,
    pub lfQuality: BYTE,
    pub lfPitchAndFamily: BYTE,
    pub lfFaceName: LPCSTR,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// WIN32 FUNCTIONS
///////////////////////////////////////////////////////////////////////////////////////////////////

pub type WNDPROC =
    unsafe extern "system" fn(param0: HWND, param1: u32, param2: WPARAM, param3: LPARAM) -> LRESULT;

#[link(name = "Kernel32")]
extern "system" {
    pub fn GetModuleHandleA(lpmodulename: PCSTR) -> HINSTANCE;
    pub fn CloseHandle(hObject: HANDLE) -> BOOL;

    pub fn CreateFileA(
        lpFileName: LPCSTR,
        dwDesireAccess: DWORD,
        dwShareMode: DWORD,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        dwCreationDisposition: DWORD,
        dwFlagsAndAttributes: DWORD,
        hTemplateFile: HANDLE,
    ) -> HANDLE;

    pub fn SetFilePointer(
        hFile: HANDLE,
        lDistanceToMove: LONG,
        lpDistanceToMoveHigh: PLONG,
        dwMoveMethod: DWORD,
    ) -> DWORD;

    pub fn ReadFile(
        hFile: HANDLE,
        lpBuffer: LPVOID,
        nNumberOfBytesToRead: DWORD,
        lpNumberOfBytesRead: LPDWORD,
        lpOverLapped: LPOVERLAPPED,
    ) -> BOOL;
}

#[link(name = "User32")]
extern "system" {
    pub fn LoadCursorW(hinstance: HINSTANCE, lpcursorname: PCWSTR) -> HCURSOR;
    pub fn SetCursor(hcursor: HCURSOR) -> HCURSOR;
    pub fn ShowCursor(bshow: BOOL) -> i32;
    pub fn LoadCursorFromFileA(lpFileName: LPCWSTR) -> HCURSOR;
    pub fn RegisterClassA(lpwndclass: *const WNDCLASSA) -> u16;
    pub fn ValidateRect(hwnd: HWND, lprect: *const RECT) -> BOOL;
    pub fn PostQuitMessage(nexitcode: i32);
    pub fn TranslateMessage(lpmsg: *const MSG) -> BOOL;
    pub fn DispatchMessageA(lpmsg: *const MSG) -> LRESULT;
    pub fn GetClientRect(hWnd: HWND, lpRect: *const RECT) -> BOOL;
    pub fn GetWindowRect(hWnd: HWND, lpRect: *const RECT) -> BOOL;
    pub fn BeginPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> HDC;
    pub fn EndPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;
    pub fn ReleaseDC(hwnd: HWND, hdc: HDC) -> i32;
    pub fn DefWindowProcA(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT;
    pub fn GetDC(hwnd: HWND) -> HDC;
    pub fn SetWindowLongPtrA(hwnd: HWND, nindex: i32, dwnewlong: LONG_PTR) -> LONG_PTR;
    pub fn GetWindowLongPtrA(hwnd: HWND, nindex: i32) -> LONG_PTR;
    pub fn GetCursorPos(lppoint: *const POINT) -> BOOL;
    pub fn ScreenToClient(hwnd: HWND, lppoint: *const POINT) -> BOOL;

    pub fn CreateWindowExA(
        dwexstyle: WINDOW_EX_STYLE,
        lpclassname: PCSTR,
        lpwindowname: PCSTR,
        dwstyle: WINDOW_STYLE,
        x: i32,
        y: i32,
        nwidth: i32,
        nheight: i32,
        hwndparent: HWND,
        hmenu: HMENU,
        hinstance: HINSTANCE,
        lpparam: *const std::ffi::c_void,
    ) -> HWND;

    pub fn MessageBoxA(
        hwnd: HWND,
        lptext: PCSTR,
        lpcaption: PCSTR,
        utype: MESSAGEBOX_STYLE,
    ) -> MESSAGEBOX_RESULT;

    pub fn GetMessageA(lpmsg: *mut MSG, hwnd: HWND, wmsgfiltermin: u32, wmsgfiltermax: u32)
        -> BOOL;

    pub fn PeekMessageA(
        lpMsg: *mut MSG,
        hWnd: HWND,
        wMsgFilterMin: UINT,
        wMsgFilterMax: UINT,
        wRemoveMsg: UINT,
    ) -> BOOL;

    pub fn AdjustWindowRectEx(
        lpRect: *const RECT,
        dwStyle: DWORD,
        bMenu: BOOL,
        dwExStyle: DWORD,
    ) -> BOOL;

    pub fn SetWindowPos(
        hwnd: HWND,
        hwndInsertAfter: HWND,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        uFlags: UINT,
    ) -> BOOL;
}

#[link(name = "Gdi32")]
extern "system" {
    pub fn PatBlt(hdc: HDC, x: i32, y: i32, w: i32, h: i32, rop: DWORD) -> BOOL;
    pub fn DeleteObject(ho: HGDIOBJ) -> BOOL;
    pub fn CreateCompatibleDC(hdc: HDC) -> HDC;

    pub fn CreateDIBSection(
        hdc: HDC,
        pbmi: *const BITMAPINFO,
        usage: DIB_USAGE,
        ppvbits: *mut *mut c_void,
        hsection: HANDLE,
        offset: u32,
    ) -> HBITMAP;

    pub fn StretchDIBits(
        hdc: HDC,
        xDest: i32,
        yDest: i32,
        destWidth: i32,
        destHeight: i32,
        xSrc: i32,
        ySrc: i32,
        srcWidth: i32,
        srcHeight: i32,
        lpBits: *const c_void,
        lpBmi: *const BITMAPINFO,
        iusage: DIB_USAGE,
        rop: ROP_CODE,
    ) -> i32;

    pub fn CreateFontA(
        cHeight: i32,
        cWidth: i32,
        cEscapement: i32,
        cOrientation: i32,
        cWeight: i32,
        bItalic: DWORD,
        bUnderline: DWORD,
        bStrikeOut: DWORD,
        iCharSet: DWORD,
        iOutPrecision: DWORD,
        iClipPrecision: DWORD,
        iQuality: DWORD,
        iPitchAndFamily: DWORD,
        pszFaceName: LPCSTR,
    ) -> HFONT;

    pub fn SelectObject(hdc: HDC, h: HGDIOBJ) -> HGDIOBJ;
    pub fn CreateFontIndirectA(lplf: *const LOGFONTA) -> HFONT;
}

#[link(name = "onecore")]
extern "system" {
    pub fn VirtualAlloc(
        lpAddress: LPVOID,
        dwSize: DWORD,
        flAllocationType: DWORD,
        flProtect: DWORD,
    ) -> LPVOID;

    pub fn VirtualFree(lpAddress: LPVOID, dwSize: DWORD, dwFreeType: DWORD) -> BOOL;
}

#[link(name = "Xinput")]
extern "system" {
    pub fn XInputGetState(dwUserIndex: DWORD, pState: *const XINPUT_STATE) -> DWORD;
    pub fn XInputSetState(dwUserIndex: DWORD, pVibration: *const XINPUT_VIBRATION) -> DWORD;
}

#[link(name = "Ole32")]
extern "system" {
    pub fn CoInitializeEx(pvReserved: LPVOID, dwCoInit: DWORD) -> HRESULT;
}

#[link(name = "Dwmapi")]
extern "system" {
    pub fn DwmGetWindowAttribute(
        hwnd: HWND,
        dwAttribute: DWORD,
        pvAttribute: PVOID,
        cbAttribute: DWORD,
    ) -> HRESULT;
}
