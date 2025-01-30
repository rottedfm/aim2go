use std::ptr::null_mut;
use winapi::um::winuser::*;
use winapi::um::wingdi::*;
use winapi::shared::windef::*;
use winapi::shared::minwindef::*;
use winapi::um::libloaderapi::GetModuleHandleW;

/// Function to get the size and position of the game window
fn get_game_window_rect(hwnd: HWND) -> Option<RECT> {
    let mut rect: RECT = unsafe { std::mem::zeroed() };
    unsafe {
        if GetWindowRect(hwnd, &mut rect) != 0 {
            Some(rect)
        } else {
            None
        }
    }
}

/// Window procedure to handle overlay drawing
unsafe extern "system" fn window_proc(hwnd: HWND, msg: UINT, _wparam: WPARAM, _lparam: LPARAM) -> LRESULT {
    match msg {
        WM_PAINT => {
            draw_dot(hwnd);
            0
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, _wparam, _lparam),
    }
}

/// Function to draw a **dot** at the center of the overlay window
unsafe fn draw_dot(hwnd: HWND) {
    let mut ps: PAINTSTRUCT = std::mem::zeroed();
    let hdc = BeginPaint(hwnd, &mut ps);

    let mut rect: RECT = std::mem::zeroed();
    GetClientRect(hwnd, &mut rect);

    let center_x = (rect.right - rect.left) / 2;
    let center_y = (rect.bottom - rect.top) / 2;
    let dot_size = 4; // Radius of the dot

    // Create a red brush for the dot
    let red_brush = CreateSolidBrush(RGB(255, 0, 255));
    let old_brush = SelectObject(hdc, red_brush as *mut _);

    // Draw a small filled circle (dot)
    Ellipse(
        hdc,
        center_x - dot_size,
        center_y - dot_size,
        center_x + dot_size,
        center_y + dot_size,
    );

    // Cleanup
    SelectObject(hdc, old_brush);
    DeleteObject(red_brush as *mut _);
    EndPaint(hwnd, &ps);
}

/// Function to create an **overlay** that matches the game window's size
pub fn create_overlay(game_window: HWND) {
    unsafe {
        if let Some(game_rect) = get_game_window_rect(game_window) {
            let h_instance = GetModuleHandleW(null_mut());
            let class_name = "OverlayWindow\0".encode_utf16().collect::<Vec<u16>>();

            let wnd_class = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(window_proc),
                hInstance: h_instance,
                hCursor: LoadCursorW(null_mut(), IDC_ARROW),
                hbrBackground: CreateSolidBrush(RGB(0, 0, 0)), // Black background
                lpszClassName: class_name.as_ptr(),
                ..std::mem::zeroed()
            };

            RegisterClassW(&wnd_class);

            let hwnd = CreateWindowExW(
                WS_EX_LAYERED | WS_EX_TOPMOST | WS_EX_TRANSPARENT | WS_EX_TOOLWINDOW,
                class_name.as_ptr(),
                "Game Overlay\0".encode_utf16().collect::<Vec<u16>>().as_ptr(),
                WS_POPUP | WS_VISIBLE,
                game_rect.left,
                game_rect.top,
                game_rect.right - game_rect.left,
                game_rect.bottom - game_rect.top,
                null_mut(),
                null_mut(),
                h_instance,
                null_mut(),
            );

            // Make the window transparent except for the dot
            SetLayeredWindowAttributes(hwnd, RGB(0, 0, 0), 0, LWA_COLORKEY);

            let mut msg: MSG = std::mem::zeroed();
            while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        } else {
            eprintln!("Failed to get game window size.");
        }
    }
}
