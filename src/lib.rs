#![forbid(unsafe_code)]
// TODO #![deny(missing_docs)]

pub mod components;
pub mod methods;
pub mod setup;
pub mod utils;

pub use setup::TwElementsSetup;

#[cfg(feature = "ssr")]
#[cfg(feature = "axum")]
pub use setup::AxumRouterExt;

mod build;
pub use build::install_files_to;
