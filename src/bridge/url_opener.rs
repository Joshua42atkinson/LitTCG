// bridge/url_opener.rs — Cross-platform URL opening
use bevy::prelude::*;

pub const PURCHASE_URL: &str = "https://polar.sh/your-product";

/// Opens the purchase URL in the user's default browser where supported.
pub fn open_purchase_url() {
    info!("Purchase URL: {}", PURCHASE_URL);

    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let _ = window.location().set_href(PURCHASE_URL);
        }
    }
}
