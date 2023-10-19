pub struct MediaSessionManager {
    yes: String,
}

impl MediaSessionManager {
    pub fn new() -> MediaSessionManager {

        MediaSessionManager {
            yes: "yes".into(),
        }
    }
}