// MIT License - Copyright (c) 2026 ScreenPrompt Contributors
// macOS API integration for ScreenPrompt using AppKit/Cocoa

use tauri::WebviewWindow;
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;

const NS_WINDOW_SHARING_NONE: isize = 0;

/// Exclude window from screenshots and screen sharing, and configure it to float above fullscreen apps.
pub fn apply_capture_exclusion(window: WebviewWindow) -> Result<(), String> {
    unsafe {
        let ns_window_ptr = window.ns_window().map_err(|e| format!("Failed to get NSWindow: {}", e))?;
        if ns_window_ptr.is_null() {
            return Err("NSWindow pointer is null".to_string());
        }
        let ns_window = ns_window_ptr as *mut Object;
        
        // Exclude window from capture
        let _: () = msg_send![ns_window, setSharingType: NS_WINDOW_SHARING_NONE];

        // Set window level to float above fullscreen apps (NSScreenSaverWindowLevel: 1000)
        let level: isize = 1000;
        let _: () = msg_send![ns_window, setLevel: level];

        // Combine NSWindowCollectionBehaviorCanJoinAllSpaces (1) and NSWindowCollectionBehaviorFullScreenAuxiliary (256)
        let collection_behavior: usize = 1 | 256;
        let _: () = msg_send![ns_window, setCollectionBehavior: collection_behavior];

        Ok(())
    }
}

/// Toggle mouse pass-through mode
pub fn set_click_through(window: WebviewWindow, enabled: bool) -> Result<(), String> {
    unsafe {
        let ns_window_ptr = window.ns_window().map_err(|e| format!("Failed to get NSWindow: {}", e))?;
        if ns_window_ptr.is_null() {
            return Err("NSWindow pointer is null".to_string());
        }
        let ns_window = ns_window_ptr as *mut Object;
        let ignores: objc::runtime::BOOL = if enabled { objc::runtime::YES } else { objc::runtime::NO };
        let _: () = msg_send![ns_window, setIgnoresMouseEvents: ignores];
    }

    // Toggle Tauri/webview level click pass-through
    window
        .set_ignore_cursor_events(enabled)
        .map_err(|e| e.to_string())?;

    Ok(())
}
