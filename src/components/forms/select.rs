use leptos::*;
use std::sync::Arc;
use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen};
use web_sys::HtmlSelectElement;

// TODO More flexibility, implement remaining tailwind elements features

/// Implement this for a type, e.g. an enum, to make this type usable as an option in a select input.
pub trait SelectOption {
    /// A string representation of the option value, used in the `value` attribute of the `<option>` tag.
    /// These must be unique and uniquely identify the option.
    fn value(&self) -> String;
    /// How to display the option in the select input.
    fn view(&self) -> View;
}

/// A Select component
///
/// See [Tailwind Elements: Select](https://tailwind-elements.com/docs/standard/forms/select/)
#[component]
pub fn Select<O, OnChangeFn>(
    /// The id of the select input. Useful to associate a label with it.
    // TODO Auto-assign id
    #[prop(into, default = "".into())]
    id: String,
    #[prop(into)] label: String,
    // TODO Make options reactive?
    #[prop(into)] options: Vec<O>,
    /// Set the current value
    #[prop(into)]
    value: Signal<O>,
    /// Callback that is called when the selection is changed.
    on_change: OnChangeFn,
) -> impl IntoView
where
    O: SelectOption + Clone + 'static,
    OnChangeFn: Fn(&O) + Clone + 'static,
{
    // TODO This explicit initialization is a workaround for https://github.com/mdbootstrap/Tailwind-Elements/issues/1743
    let element_ref: NodeRef<leptos::html::Select> = create_node_ref();
    let options = store_value(options);
    create_effect(move |_| {
        if let Some(element) = element_ref() {
            let jsselect = JsSelect::new(&element);

            let on_change = on_change.clone();
            let on_value_change = Closure::new(move |option_value: String| {
                options.with_value(|options| {
                    let mut options_iter = options
                        .iter()
                        .filter(|option| option.value() == option_value);
                    let Some(value) = options_iter.next() else {
                        panic!("Select value changed to an invalid value: {}", option_value);
                    };
                    if options_iter.next().is_some() {
                        panic!(
                            "Select value changed to a duplicate value: {}",
                            option_value
                        );
                    };
                    on_change(value);
                })
            });

            let jsselect = Arc::new(jsselect);
            let jsselect_clone = Arc::clone(&jsselect);
            // TODO Is this effect cleaned up correctly? What if the outer effect is re-run and this gets re-created?
            create_effect(move |_| {
                jsselect_clone.setValue(value().value());
            });

            // TODO leptos_use has an addEventListener function that automatically cleans itself up on scope exit. We probably should use that.
            te_select_add_event_listener(&element, "valueChange.te.select", &on_value_change);

            on_cleanup(move || {
                jsselect.dispose();
                std::mem::drop(on_value_change);
            });
        }
    });

    options.with_value(|options| {
        view! {
            <div>
                // TODO Why is the `data-te-select-init` needed? Without it, layout of the label is broken.
                <select data-te-select-init id=id.clone() ref=element_ref>
                    {options.into_iter().map(|option| {
                        view! {
                            <option value=option.value()>{option.view()}</option>
                        }
                    }).collect::<Vec<_>>()}
                </select>
                <label for=id data-te-select-label-ref>{label}</label>
            </div>
        }
    })
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = te, js_name = Select)]
    type JsSelect;

    // TODO Select constructor can take some options, see https://tailwind-elements.com/docs/standard/forms/select/#docsTabsAPI
    #[wasm_bindgen(constructor, js_namespace = te, js_class = Select, final)]
    fn new(e: &HtmlSelectElement) -> JsSelect;

    #[wasm_bindgen(method, js_namespace = te, js_class = Select, final)]
    fn setValue(this: &JsSelect, value: String);

    #[wasm_bindgen(method, js_namespace = te, js_class = Select, final)]
    fn dispose(this: &JsSelect);
}

#[wasm_bindgen(
    inline_js = "export function te_select_add_event_listener(select_html_event, event_name, callback) { select_html_event.addEventListener(event_name, (event) => {
        callback(event.target.value);
    }); }"
)]
extern "C" {
    #[wasm_bindgen]
    fn te_select_add_event_listener(
        carousel: &web_sys::HtmlElement,
        event_name: &str,
        callback: &Closure<dyn FnMut(String)>,
    );
}
