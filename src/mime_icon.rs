// SPDX-License-Identifier: GPL-3.0-only

use cosmic::widget::icon;
use mime_guess::Mime;
use rustc_hash::FxHashMap;
use std::{
    fs,
    path::Path,
    sync::{LazyLock, Mutex},
};

use crate::custom_icons;

pub const FALLBACK_MIME_ICON: &str = "text-x-generic";

#[derive(Debug, Eq, Hash, PartialEq)]
struct MimeIconKey {
    mime: Mime,
    size: u16,
}

struct MimeIconCache {
    cache: FxHashMap<MimeIconKey, Option<icon::Handle>>,
    shared_mime_info: xdg_mime::SharedMimeInfo,
}

impl MimeIconCache {
    pub fn new() -> Self {
        Self {
            cache: FxHashMap::default(),
            shared_mime_info: xdg_mime::SharedMimeInfo::new(),
        }
    }

    pub fn get(&mut self, key: MimeIconKey) -> Option<icon::Handle> {
        self.cache
            .entry(key)
            .or_insert_with_key(|key| {
                // Try custom icons first
                if let Some(custom_handle) =
                    custom_icons::get_custom_icon(key.mime.as_ref(), key.size)
                {
                    return Some(custom_handle);
                }

                // Fall back to system icon theme
                let mut icon_names = self.shared_mime_info.lookup_icon_names(&key.mime);
                if icon_names.is_empty() {
                    // No icon names found - try generic fallback based on MIME type category
                    let type_str = key.mime.type_().as_str();
                    let generic_icon = match type_str {
                        "text" => "text-x-generic",
                        "image" => "image-x-generic",
                        "audio" => "audio-x-generic",
                        "video" => "video-x-generic",
                        "application" => "application-x-executable",
                        "inode" => "folder",
                        _ => FALLBACK_MIME_ICON,
                    };

                    // Try custom_icons for the generic type first
                    if let Some(custom_handle) =
                        custom_icons::ui::get_icon(generic_icon, key.size)
                    {
                        return Some(custom_handle);
                    }

                    // Use system icon with comprehensive fallback chain
                    return Some(
                        icon::from_name(generic_icon)
                            .size(key.size)
                            .fallback(Some(icon::IconFallback::Names(vec![
                                FALLBACK_MIME_ICON.into(),
                                "application-octet-stream".into(),
                                "unknown".into(),
                            ])))
                            .handle(),
                    );
                }
                let icon_name = icon_names.remove(0);
                let mut named = icon::from_name(icon_name).size(key.size);
                // Add fallback chain including the remaining icon names plus generic fallbacks
                let mut fallback_names: Vec<std::borrow::Cow<'_, str>> =
                    icon_names.into_iter().map(std::borrow::Cow::from).collect();
                // Always add generic fallbacks at the end
                fallback_names.extend([
                    FALLBACK_MIME_ICON.into(),
                    "application-octet-stream".into(),
                ]);
                named = named.fallback(Some(icon::IconFallback::Names(fallback_names)));
                Some(named.handle())
            })
            .clone()
    }
}
static MIME_ICON_CACHE: LazyLock<Mutex<MimeIconCache>> =
    LazyLock::new(|| Mutex::new(MimeIconCache::new()));

pub fn mime_for_path(
    path: impl AsRef<Path>,
    metadata_opt: Option<&fs::Metadata>,
    remote: bool,
) -> Mime {
    let path = path.as_ref();
    let mime_icon_cache = MIME_ICON_CACHE.lock().unwrap();
    // Try the shared mime info cache first
    let mut gb = mime_icon_cache.shared_mime_info.guess_mime_type();
    if remote {
        if let Some(file_name) = path.file_name().and_then(std::ffi::OsStr::to_str) {
            gb.file_name(file_name);
        }
    } else {
        gb.path(path);
    }
    if let Some(metadata) = metadata_opt {
        gb.metadata(metadata.clone());
    }
    let guess = gb.guess();
    let guessed_mime = guess.mime_type();

    /// Checks if the `Mime` is a special variant that should not use extension fallback.
    fn is_non_file_mime(mime: &Mime) -> bool {
        *mime == "inode/directory" || *mime == "inode/symlink"
    }

    /// Checks if the file is empty/zero-size
    fn is_empty_file_mime(mime: &Mime) -> bool {
        *mime == "application/x-zerosize" || *mime == "inode/x-empty"
    }

    // For empty files, always fall back to extension-based detection
    if is_empty_file_mime(guessed_mime) {
        mime_guess::from_path(path).first_or_octet_stream()
    } else if guess.uncertain() && (remote || !is_non_file_mime(guessed_mime)) {
        // If uncertain, try mime_guess
        mime_guess::from_path(path).first_or_octet_stream()
    } else {
        guessed_mime.clone()
    }
}

pub fn mime_icon(mime: Mime, size: u16) -> icon::Handle {
    let mut mime_icon_cache = MIME_ICON_CACHE.lock().unwrap();
    match mime_icon_cache.get(MimeIconKey { mime, size }) {
        Some(handle) => handle,
        None => {
            // Try custom icons first for guaranteed fallback
            if let Some(handle) = custom_icons::ui::get_icon(FALLBACK_MIME_ICON, size) {
                return handle;
            }
            // Last resort: system icon with fallback chain
            icon::from_name(FALLBACK_MIME_ICON)
                .size(size)
                .fallback(Some(icon::IconFallback::Names(vec![
                    "application-octet-stream".into(),
                    "unknown".into(),
                ])))
                .handle()
        }
    }
}

pub fn parent_mime_types(mime: &Mime) -> Option<Vec<Mime>> {
    let mime_icon_cache = MIME_ICON_CACHE.lock().unwrap();

    mime_icon_cache.shared_mime_info.get_parents_aliased(mime)
}
