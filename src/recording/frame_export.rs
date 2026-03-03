//! Frame-by-frame ANSI export for post-processing.
//! This module provides optional `--export-frames` functionality
//! that writes each rendered frame as an ANSI text file.
//!
//! This is a future enhancement — not yet implemented.

use std::path::Path;

use color_eyre::Result;

/// Export configuration.
#[allow(dead_code)]
pub struct FrameExporter {
    output_dir: String,
    frame_count: u64,
}

#[allow(dead_code)]
impl FrameExporter {
    pub fn new(output_dir: &str) -> Result<Self> {
        std::fs::create_dir_all(output_dir)?;
        Ok(Self {
            output_dir: output_dir.to_string(),
            frame_count: 0,
        })
    }

    /// Export a single frame's buffer content as ANSI text.
    pub fn export_frame(&mut self, _buf: &ratatui::buffer::Buffer) -> Result<()> {
        let _path = Path::new(&self.output_dir).join(format!("frame_{:06}.txt", self.frame_count));
        self.frame_count += 1;
        // TODO: Implement ANSI text export from buffer
        Ok(())
    }
}
