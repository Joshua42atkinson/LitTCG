// bridge/tts_client.rs — Kokoro TTS sidecar client (native/desktop only)
#![cfg(all(feature = "tts", not(target_arch = "wasm32")))]

pub const KOKORO_ENDPOINT: &str = "http://localhost:8200/v1/audio/speech";

/// Sends a TTS request to the Kokoro sidecar and writes the resulting MP3 to disk.
pub fn request_tts_audio(text: String, voice: String) {
    bevy::tasks::IoTaskPool::get().spawn(async move {
        let client = reqwest::Client::new();
        let payload = serde_json::json!({
            "model": "kokoro",
            "input": text,
            "voice": voice,
            "response_format": "mp3"
        });

        match client.post(KOKORO_ENDPOINT)
            .json(&payload)
            .send()
            .await
        {
            Ok(resp) => {
                if resp.status().is_success() {
                    if let Ok(bytes) = resp.bytes().await {
                        use std::io::Write;
                        if let Ok(mut file) = std::fs::File::create(crate::asset_catalog::TTS_OUTPUT_PATH) {
                            let _ = file.write_all(&bytes);
                        }
                    }
                } else {
                    tracing::warn!("Kokoro TTS sidecar returned error status: {}", resp.status());
                }
            }
            Err(e) => {
                tracing::warn!("Failed to contact Kokoro TTS sidecar (is it offline?): {}", e);
            }
        }
    }).detach();
}
