use super::super::utils::{HtmlElementAttributeExt, MaybeSignalExt};
use leptos::{html::ElementDescriptor, *};

/// Add a ripple effect to [Button](crate::components::Button) or other components
///
/// See <https://tailwind-elements.com/docs/standard/methods/ripple>
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ripple {
    pub color: Option<String>,
    pub duration: Option<String>,
    pub centered: bool,
    pub unbound: bool,
    pub radius: Option<u32>,
}

impl Ripple {
    pub(crate) fn apply<T: ElementDescriptor + 'static>(
        cx: Scope,
        ripple: impl Into<MaybeSignal<Option<Ripple>>>,
        element: HtmlElement<T>,
    ) -> HtmlElement<T> {
        let ripple = ripple.into();
        element
            .attr_valueless(
                "data-te-ripple-init",
                ripple.map(cx, |ripple| ripple.is_some()),
            )
            .attr(
                "data-te-ripple-color",
                ripple.map(cx, |ripple| {
                    ripple.as_ref().and_then(|ripple| ripple.color.clone())
                }),
            )
            .attr(
                "data-te-ripple-duration",
                ripple.map(cx, |ripple| {
                    ripple.as_ref().and_then(|ripple| ripple.duration.clone())
                }),
            )
            .attr_bool(
                "data-te-ripple-centered",
                ripple.map(cx, |ripple| {
                    ripple
                        .as_ref()
                        .map(|ripple| ripple.centered)
                        .unwrap_or(false)
                }),
            )
            .attr_bool(
                "data-te-ripple-unbound",
                ripple.map(cx, |ripple| {
                    ripple
                        .as_ref()
                        .map(|ripple| ripple.unbound)
                        .unwrap_or(false)
                }),
            )
            .attr(
                "data-te-ripple-radius",
                ripple.map(cx, |ripple| {
                    ripple
                        .as_ref()
                        .and_then(|ripple| ripple.radius)
                        .map(|r| r.to_string())
                }),
            )
    }
}
