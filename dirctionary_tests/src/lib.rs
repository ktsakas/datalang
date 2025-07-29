pub mod base {
    include!("../macro_definitions/base.rs");
}

pub mod social_media {
    include!("../macro_definitions/social_media.rs");
}

pub use base::*;
pub use social_media::*;