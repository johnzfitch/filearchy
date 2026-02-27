// SPDX-License-Identifier: GPL-3.0-only

use cosmic::widget::icon;
use rust_embed::RustEmbed;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Try to load a custom icon as Handle (for most UI widgets)
pub fn handle_or_default(name: impl Into<std::sync::Arc<str>>, size: u16) -> icon::Handle {
    let name_str = name.into();

    // Try custom icon first
    if let Some(handle) = ui::get_icon(&name_str, size) {
        return handle;
    }

    // Fall back to system icon theme
    icon::from_name(name_str).size(size).handle()
}

/// Try to load a custom icon as Icon widget (for menu items)
pub fn icon_or_default(name: impl Into<std::sync::Arc<str>>, size: u16) -> icon::Icon {
    let name_str = name.into();

    // Try custom icon first
    if let Some(handle) = ui::get_icon(&name_str, size) {
        return icon::icon(handle).size(size);
    }

    // Fall back to system icon theme
    icon::from_name(name_str).size(size).icon()
}

#[derive(RustEmbed)]
#[folder = "resources/icons/"]
pub struct CustomIcons;

/// Mapping of MIME type to custom icon filename
static MIME_ICON_MAP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    // Programming Languages
    map.insert("application/javascript", "application-javascript.png");
    map.insert("application/x-javascript", "application-x-javascript.png");
    map.insert("text/javascript", "text-javascript.png");
    map.insert("application/typescript", "application-typescript.png");
    map.insert("text/typescript", "text-typescript.png");
    map.insert("text/x-typescript", "text-x-typescript.png");
    map.insert("application/x-python", "application-x-python.png");
    map.insert("text/x-python", "text-x-python.png");
    map.insert("application/x-shellscript", "application-x-shellscript.png");
    map.insert("text/x-shellscript", "text-x-shellscript.png");
    map.insert("application/sql", "application-sql.png");
    map.insert("text/x-sql", "text-x-sql.png");
    map.insert("text/rust", "text-rust.png");
    map.insert("text/x-rust", "text-x-rust.png");
    map.insert("text/x-c", "blue-document-attribute-c-16x16.png");
    map.insert("text/x-c++", "blue-document-attribute-c-16x16.png");
    map.insert("text/x-java", "coffee-ilny-emoji-cup-48x48.png");
    map.insert("text/x-go", "script-48x48.png");
    map.insert("application/x-ruby", "script-48x48.png");
    map.insert("text/x-php", "script-48x48.png");

    // Web & Markup
    map.insert("text/html", "text-html.png");
    map.insert("application/xhtml+xml", "application-xhtml+xml.png");
    map.insert("text/css", "text-css.png");
    map.insert("text/markdown", "text-markdown.png");
    map.insert("text/x-markdown", "text-x-markdown.png");

    // Data Formats
    map.insert("application/json", "application-json.png");
    map.insert("text/yaml", "text-yaml.png");
    map.insert("text/x-yaml", "text-x-yaml.png");
    map.insert("application/x-yaml", "application-x-yaml.png");
    map.insert("application/toml", "options-64x64.png");
    map.insert("text/x-ini", "time-settings-64x64.png");
    map.insert("text/csv", "text-csv.png");
    map.insert("text/x-config", "text-x-config.png");
    map.insert("application/x-config", "application-x-config.png");
    map.insert("application/xml", "blue-document-code-16x16.png");
    map.insert("text/xml", "script-48x48.png");

    // Documents
    map.insert("application/pdf", "application-pdf.png");
    map.insert("application/msword", "application-msword.png");
    map.insert("application/vnd.openxmlformats-officedocument.wordprocessingml.document",
               "application-vnd.openxmlformats-officedocument.wordprocessingml.document.png");
    map.insert("application/rtf", "application-rtf.png");
    map.insert("text/rtf", "text-rtf.png");
    map.insert("application/epub+zip", "book-48x48.png");

    // Spreadsheets & Presentations
    map.insert("application/vnd.ms-excel", "application-vnd.ms-excel.png");
    map.insert("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
               "application-vnd.openxmlformats-officedocument.spreadsheetml.sheet.png");
    map.insert("application/vnd.ms-powerpoint", "application-vnd.ms-powerpoint.png");
    map.insert("application/vnd.openxmlformats-officedocument.presentationml.presentation",
               "application-vnd.openxmlformats-officedocument.presentationml.presentation.png");

    // Images
    map.insert("image/png", "picture-48x48.png");
    map.insert("image/jpeg", "picture-48x48.png");
    map.insert("image/gif", "picture-48x48.png");
    map.insert("image/webp", "picture-48x48.png");
    map.insert("image/bmp", "picture-48x48.png");
    map.insert("image/tiff", "picture-48x48.png");
    map.insert("image/svg+xml", "colour-picker-48x48.png");

    // Video
    map.insert("video/mp4", "video-32x32.png");
    map.insert("video/mpeg", "video-32x32.png");
    map.insert("video/webm", "video-32x32.png");
    map.insert("video/x-matroska", "video-32x32.png");
    map.insert("video/quicktime", "video-32x32.png");

    // Audio
    map.insert("audio/mpeg", "audio-mpeg.png");
    map.insert("audio/mp3", "audio-mp3.png");
    map.insert("audio/flac", "audio-flac.png");
    map.insert("audio/x-flac", "audio-x-flac.png");
    map.insert("audio/wav", "audio-wav.png");
    map.insert("audio/x-wav", "audio-x-wav.png");

    // Archives
    map.insert("application/zip", "box-48x48.png");
    map.insert("application/x-tar", "box-48x48.png");
    map.insert("application/gzip", "box-48x48.png");
    map.insert("application/x-bzip", "box-48x48.png");
    map.insert("application/x-bzip2", "box-48x48.png");
    map.insert("application/x-7z-compressed", "box-48x48.png");
    map.insert("application/x-rar", "box-48x48.png");
    map.insert("application/x-rar-compressed", "box-48x48.png");
    map.insert("application/zstd", "box-48x48.png");
    map.insert("application/x-zstd-compressed-tar", "box-48x48.png");
    map.insert("application/x-lz4", "box-48x48.png");
    map.insert("application/x-lz4-compressed-tar", "box-48x48.png");
    map.insert("application/x-brotli", "box-48x48.png");
    map.insert("application/x-brotli-compressed-tar", "box-48x48.png");

    // System & Special
    map.insert("application/x-executable", "application-x-executable.png");
    map.insert("application/x-ms-dos-executable", "application-x-ms-dos-executable.png");
    map.insert("application/x-pem-file", "application-x-pem-file.png");
    map.insert("application/x-x509-ca-cert", "application-x-x509-ca-cert.png");
    map.insert("text/plain", "text-plain.png");
    map.insert("text/x-log", "text-x-log.png");
    map.insert("inode/directory", "folder-48x48.png");
    map.insert("application/octet-stream", "documents-48x48.png");

    map
});

/// Get custom icon for a MIME type
pub fn get_custom_icon(mime_type: &str, size: u16) -> Option<icon::Handle> {
    use image::imageops::FilterType;

    if let Some(filename) = MIME_ICON_MAP.get(mime_type) {
        let path = format!("mimetypes/{filename}");
        if let Some(file_data) = CustomIcons::get(&path) {
            // Decode and resize to requested size
            if let Ok(img) = image::load_from_memory(&file_data.data) {
                let native_size = img.width();

                // Only scale if the requested size differs from native size
                if size as u32 != native_size {
                    let resized = img.resize_exact(
                        size as u32,
                        size as u32,
                        FilterType::Lanczos3
                    );
                    let rgba = resized.into_rgba8();
                    return Some(icon::from_raster_pixels(
                        size as u32,
                        size as u32,
                        rgba.into_raw()
                    ));
                }
            }

            // Fallback to unscaled if size matches or decode fails
            return Some(icon::from_raster_bytes(file_data.data));
        }
    }
    None
}

/// UI icon names for common interface elements
pub mod ui {
    use super::*;
    use image::imageops::FilterType;

    pub fn get_icon(name: &str, size: u16) -> Option<icon::Handle> {
        // Strip -symbolic suffix if present (GNOME/freedesktop convention)
        let name = name.strip_suffix("-symbolic").unwrap_or(name);

        let filename = match name {
            // Navigation
            "go-up" | "up" => "arrow-up-48x48.png",
            "go-down" | "down" => "arrow-down-48x48.png",
            "go-previous" | "back" => "back-32x32.png",
            "go-next" | "forward" => "forward-32x32.png",

            // File operations
            "edit-copy" | "copy" => "copy-64x64.png",
            "edit-cut" | "cut" => "cut-64x64.png",
            "edit-paste" | "paste" => "paste-32x32-64x64.png",
            "edit-delete" | "delete" => "dark-edit-delete-32x32.png",
            "user-trash" => "dark-edit-delete-32x32.png",
            "user-trash-full" => "dark-edit-delete-32x32.png",
            "document-open" | "open" => "open-file-48x48-64x64.png",

            // Edit
            "edit" | "document-edit" => "edit-24x24.png",
            "edit-clear" => "close-24x24.png",

            // View
            "view-grid" | "grid" => "table-64x64.png",
            "view-list" | "list" => "list-24x24.png",
            "search" | "edit-find" | "system-search" => "search-24x24.png",
            "view-refresh" | "refresh" => "update-30-30x30.png",
            "sidebar-show" | "sidebar-hide" | "view-sidebar" | "pane-show" | "pane-hide"
            | "panel-show" | "panel-hide" => "burger-menu-64x64.png",

            // System
            "go-home" | "user-home" => "house-32x32.png",
            "emblem-favorite" | "favorite" => "favourites-64x64.png",
            "preferences-system" | "settings" => "settings.png",

            // Status
            "dialog-information" | "info" => "info-64x64.png",
            "dialog-warning" | "warning" => "warning-24x24.png",
            "dialog-error" | "error" => "error-64x64.png",
            "emblem-ok" | "ok" | "apply" => "apply-24x24.png",
            "dialog-question" => "question-24x24.png",
            "vcs-branch" => "hierarchy-24x24.png",
            "text-x-generic" => "text-plain.png",

            // New icons - Navigation & Window
            "window-close" => "close-64x64.png",
            "view-fullscreen" => "sizes-32x32.png",

            // New icons - File Operations
            "folder-new" => "add-folder-32x32.png",
            "document-new" => "new-file-64x64.png",
            "utilities-terminal" => "dark-utilities-terminal-32x32.png",
            "edit-select-all" => "checkbox-64x64.png",
            "mark-location" => "kensicons-emblem-favorite-48x48.png",
            "starred" => "kensicons-emblem-favorite-48x48.png",

            // New icons - View & Sort
            "view-more" => "burger-menu-64x64.png",
            "format-text-direction-ltr" => "unordered-list-64x64.png", // Text sort
            "view-sort-descending" => "arrow-down-48x48.png",
            "view-sort-ascending" => "arrow-up-48x48.png",
            "pan-down" => "arrow-down-48x48.png",
            "pan-up" => "arrow-up-48x48.png",

            // New icons - System & Network
            "network-workgroup" => "wireless-connection-64x64.png",
            "media-eject" => "eject-24x24.png",
            "checkbox-checked" => "checkbox-64x64.png",
            "document-open-recent" => "clock-history-16x16.png",

            // New icons - Special Folders
            "folder-documents" => "documents-48x48.png",
            "folder-download" => "folder-downloads-64x64.png",
            "folder-music" => "music-folder-64x64.png",
            "folder-pictures" => "camera-camera-capture-64x64.png",
            "folder-videos" => "movies-64x64.png",
            "folder-templates" => "documents-48x48.png", // Using documents
            "folder-publicshare" => "wireless-connection-64x64.png", // Using network
            "user-desktop" => "monitor-24x24-64x64.png",
            "folder" => "folder-48x48.png",

            // New icons - Git Status Emblems
            "emblem-important" => "warning-24x24.png",
            "emblem-default" => "apply-24x24.png",

            // New icons - Media Controls
            "media-playback-start" => "play-24x24-64x64.png",
            "media-playback-pause" => "pause-24x24.png",

            // Additional system icons
            "document-properties" => "settings.png",

            _ => return None,
        };

        let path = format!("ui/{filename}");
        if let Some(file_data) = CustomIcons::get(&path) {
            // Decode and resize to requested size
            if let Ok(img) = image::load_from_memory(&file_data.data) {
                let native_size = img.width();

                // Only scale if the requested size differs from native size
                if size as u32 != native_size {
                    let resized = img.resize_exact(
                        size as u32,
                        size as u32,
                        FilterType::Lanczos3
                    );
                    let rgba = resized.into_rgba8();
                    return Some(icon::from_raster_pixels(
                        size as u32,
                        size as u32,
                        rgba.into_raw()
                    ));
                }
            }

            // Fallback to unscaled if size matches or decode fails
            return Some(icon::from_raster_bytes(file_data.data));
        }
        None
    }
}
