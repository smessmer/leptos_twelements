//! A UI component library for [Leptos](https://leptos.dev/), based on [Tailwind Elements](https://tailwind-elements.com/).
//! See [README](https://github.com/smessmer/leptos_twelements/README.md)

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod components;
pub mod methods;
mod setup;
pub mod utils;

pub use setup::TwElementsSetup;

#[cfg(feature = "ssr")]
#[cfg(feature = "axum")]
pub use setup::AxumRouterExt;

mod build;
pub use build::install_files_to;
