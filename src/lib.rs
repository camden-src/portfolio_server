pub mod configuration;
pub use crate::configuration::ServerSettings;

pub mod media_files;
pub use crate::media_files::{Track, list_media_files};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_succeed() {
        assert_eq!(2, 2);
    }
}
