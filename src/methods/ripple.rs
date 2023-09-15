use leptos::{html::ElementDescriptor, *};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use web_sys::HtmlElement;

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
    /// Apply a ripple effect to a node given by a [NodeRef].
    ///
    /// This works with reactive ripples. That means:
    /// - If the `ripple` signal passed as an attribute becomes `Some, then the ripple effect will
    ///   automatically be applied to the HTML element using that [NodeRef].
    /// - If the `ripple` signal passed as an attribute becomes `None`, then the ripple effect will
    ///   automatically be removed from the HTML element using that [NodeRef].
    ///
    /// Example
    /// -------
    /// ```
    /// use leptos::{html::Button, *};
    /// use leptos_twelements::methods::Ripple;
    ///
    /// #[component]
    /// pub fn MyButton() -> impl IntoView {
    ///    let ripple = Ripple::default();
    ///    let button_ref = Ripple::apply(Some(ripple));
    ///    view!{
    ///       <Button ref=button_ref>{"Click me"}</Button>
    ///    }
    /// }
    /// ```
    pub(crate) fn apply<T: ElementDescriptor + Clone + 'static>(
        element_ref: NodeRef<T>,
        ripple: impl Into<MaybeSignal<Option<Ripple>>>,
    ) {
        let ripple: MaybeSignal<Option<Ripple>> = ripple.into();

        create_effect(move |ripple_js_object: Option<Option<JsRipple>>| {
            // If there's any previous ripple set up, first dispose it
            if let Some(Some(ripple_js_object)) = ripple_js_object {
                // TODO Using the button after disposing an existing ripple
                //      currently causes Tailwind Elements to log an error.
                ripple_js_object.dispose();
            }

            if let Some(element) = element_ref() {
                ripple.with(|ripple| {
                    if let Some(ripple) = ripple {
                        let options = serde_wasm_bindgen::to_value(&ripple.options()).unwrap();
                        let ripple = JsRipple::new(&element.into_any(), options);
                        Some(ripple)
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        });
    }

    fn options(&self) -> JsRippleOptions {
        JsRippleOptions {
            color: self.color.clone(),
            duration: self.duration.clone(),
            centered: self.centered,
            unbound: self.unbound,
            radius: self.radius,
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = te, js_name = Ripple)]
    type JsRipple;

    #[wasm_bindgen(constructor, js_namespace = te, js_class = Ripple, final)]
    // TODO Pass JsRippleOptions directly, see https://github.com/cloudflare/serde-wasm-bindgen/issues/56
    fn new(e: &HtmlElement, options: JsValue) -> JsRipple;

    #[wasm_bindgen(method, js_namespace = te, js_class = Ripple, final)]
    fn dispose(this: &JsRipple);
}

#[derive(Debug, Serialize, Deserialize)]
struct JsRippleOptions {
    #[serde(rename = "rippleColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,

    #[serde(rename = "rippleDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<String>,

    #[serde(rename = "rippleCentered")]
    centered: bool,

    #[serde(rename = "rippleUnbound")]
    unbound: bool,

    #[serde(rename = "rippleRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    radius: Option<u32>,
}
