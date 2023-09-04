use super::super::utils::{HtmlElementAttributeExt, MaybeSignalExt};
use leptos::{html::ElementDescriptor, *};

/// Add a ripple effect to [Button](crate::components::Button) or other components
///
/// See [Tailwind Elements: Ripple](https://tailwind-elements.com/docs/standard/methods/ripple)
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ripple {
    /// Color of the ripple effect.
    ///
    /// See [Tailwind Elements: Ripple#Colors](https://tailwind-elements.com/docs/standard/methods/ripple/#colors)
    pub color: Option<String>,

    /// Duration of the ripple effect.
    ///
    /// See [Tailwind Elements: Ripple#Duration](https://tailwind-elements.com/docs/standard/methods/ripple/#duration)
    pub duration: Option<String>,

    /// Whether the ripple effect is centered in the component, or at the position of the click.
    ///
    /// See [Tailwind Elements: Ripple#Centered](https://tailwind-elements.com/docs/standard/methods/ripple/#centered)
    pub centered: bool,

    /// Whether the ripple effect is unbound or bound to the component (i.e. only displays within the component)
    ///
    /// See [Tailwind Elements: Ripple#Unbound](https://tailwind-elements.com/docs/standard/methods/ripple/#unbound)
    pub unbound: bool,

    /// Radius of the ripple effect.
    ///
    /// See [Tailwind Elements: Ripple#Radius](https://tailwind-elements.com/docs/standard/methods/ripple/#radius)
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
