use crate::config::REMOTE_TARGET;
use super::unwrap_ext::*;

pub fn media_url() -> String {
    REMOTE_TARGET.media_url()
}