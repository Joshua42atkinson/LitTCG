// platform_paths.rs — Per-platform save/data directories
use std::path::PathBuf;

/// Returns the directory where persistent game data should be stored.
/// On Android this is the app's internal files directory.
/// On desktop it falls back to the current working directory.
pub fn data_dir() -> PathBuf {
    #[cfg(target_os = "android")]
    {
        std::env::current_dir().unwrap_or_else(|_| {
            PathBuf::from("/data/data/com.littcg.game/files")
        })
    }
    #[cfg(not(target_os = "android"))]
    {
        std::env::current_dir().unwrap_or_default()
    }
}
