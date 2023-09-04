use leptos::*;
use leptos_meta::Script;

const JS_URL: &str = "/.twelements/twelements.js";

#[cfg(feature = "ssr")]
mod server_setup {
    use super::*;
    pub const TWELEMENTS_JS: &str =
        include_str!("../node_modules/tw-elements/dist/js/tw-elements.umd.min.js");

    #[cfg(feature = "axum")]
    mod axum_setup {
        use super::*;
        use axum::{body::HttpBody, routing::get, Router};

        pub trait AxumRouterExt {
            fn setup_twelements(self) -> Self;
        }

        #[cfg(feature = "axum")]
        impl<S, B> AxumRouterExt for Router<S, B>
        where
            B: HttpBody + Send + 'static,
            S: Clone + Send + Sync + 'static,
        {
            fn setup_twelements(self) -> Self {
                // TODO Use es module with treeshaking instead of umd module?
                // TODO Add source map support
                self.route(JS_URL, get(|| async { TWELEMENTS_JS }))
            }
        }
    }
    #[cfg(feature = "axum")]
    pub use axum_setup::AxumRouterExt;
}
#[cfg(feature = "ssr")]
#[cfg(feature = "axum")]
pub use server_setup::AxumRouterExt;

/// Add this component to your app to initialize Tailwind Elements
#[component]
pub fn TwElementsSetup(cx: Scope) -> impl IntoView {
    view! {cx,
        <Script defer="true" type_="text/javascript" src=JS_URL />
    }
}
