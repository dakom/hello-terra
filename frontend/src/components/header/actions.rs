use super::state::*;

impl Header {
    pub fn logout(&self) {
        self.app.wallet_id.set(None);
    }
}