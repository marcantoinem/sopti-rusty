use http::Uri;
use p256::PublicKey;
use web_push_native::{jwt_simple::algorithms::ES256KeyPair, Auth};

pub struct PushNotification {
    endpoint: Uri,
    ua_public: PublicKey,
    vapid_kp: ES256KeyPair,
    ua_auth: Auth,
}
