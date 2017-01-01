use ruma_identifiers::UserId;
use url::Host;

/// An active user session with a Matrix homeserver, allowing authenticated requests.
#[derive(Clone, Debug)]
pub struct Session {
    access_token: String,
    homeserver: Host,
    user_id: UserId,
}

impl Session {
    /// Create a new Session with the default attributes.
    pub fn new(access_token: String,
                homeserver: Host,
                user_id: UserId,) -> Self {
        Session {
            access_token: access_token,
            homeserver: homeserver,
            user_id: user_id
        }
    }
}
