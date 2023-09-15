use leptos::{html::Div, *};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlDivElement;

use crate::utils::MaybeSignalExt;

/// The type of an input field. This influences behavior of the input field and how its content is validated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputType {
    /// A generic text input field.
    /// See [Tailwind Elements: Inputs#Text](https://tailwind-elements.com/docs/standard/forms/inputs/#text)
    Text,

    /// An email input field.
    /// See [Tailwind Elements: Inputs#Email](https://tailwind-elements.com/docs/standard/forms/inputs/#email)
    Email,

    /// A password input field.
    /// See [Tailwind Elements: Inputs#Password](https://tailwind-elements.com/docs/standard/forms/inputs/#password)
    Password,

    /// A number input field.
    /// See [Tailwind Elements: Inputs#Number](https://tailwind-elements.com/docs/standard/forms/inputs/#number)
    Number,

    /// A telephone number input field.
    /// See [Tailwind Elements: Inputs#Tel](https://tailwind-elements.com/docs/standard/forms/inputs/#tel)
    Tel,

    /// A URL input field.
    /// See [Tailwind Elements: Inputs#Url](https://tailwind-elements.com/docs/standard/forms/inputs/#url)
    Url,
}

impl InputType {
    const fn html_attrib(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Password => "password",
            Self::Number => "number",
            Self::Tel => "tel",
            Self::Url => "url",
        }
    }
}

/// A text input component
///
/// See [Tailwind Elements: Inputs](https://tailwind-elements.com/docs/standard/forms/inputs)
#[component]
pub fn Input<OnChangeFn: Fn(String) + 'static>(
    /// This signal is read to show the current content of the input field.
    /// Whenever this signal changes, the input field is updated.
    /// But note that the input field can also be edited by the user and if the `on_change` function doesn't
    /// update the `value` signal, the input field can get out of sync with the signal.
    #[prop(into)]
    value: MaybeSignal<String>,
    /// Callback that is called when the content of the input field changes.
    on_change: OnChangeFn,
    /// Whether the input field is disabled.
    #[prop(into, default = false.into())]
    disabled: MaybeSignal<bool>,
    /// Whether the input field is readonly.
    #[prop(into, default = false.into())]
    readonly: MaybeSignal<bool>,
    /// The id of the input field. Useful to associate a label with it.
    // TODO automatically assign an id instead of taking it as an argument
    #[prop(into)]
    id: String,
    #[prop(into, default=InputType::Text.into())] input_type: MaybeSignal<InputType>,
    /// This label is shown as a placeholder while the field is empty and it is shown as a label at the top of the input field when it is non-empty or has focus.
    #[prop(into, default = "".into())]
    label: MaybeSignal<String>,
    // TODO sizing, see https://tailwind-elements.com/docs/standard/forms/inputs/
    // TODO character counter?
    // TODO Label (seems pretty neat, it moves to the top of the field), also floating labels that stay inside the field?
) -> impl IntoView {
    let class = move || {
        let mut class = "peer block min-h-[auto] w-full rounded border-0 px-3 py-[0.32rem] leading-[1.6] outline-none transition-all duration-200 ease-linear focus:placeholder:opacity-100 peer-focus:text-primary data-[te-input-state-active]:placeholder:opacity-100 motion-reduce:transition-none dark:text-neutral-200 dark:placeholder:text-neutral-200 dark:peer-focus:text-primary [&:not([data-te-input-placeholder-active])]:placeholder:opacity-0".to_string();
        if disabled() || readonly() {
            class.push_str(" bg-neutral-100 dark:bg-neutral-700");
        } else {
            class.push_str(" bg-transparent")
        }
        class
    };

    // TODO This explicit initialization is a workaround for https://github.com/mdbootstrap/Tailwind-Elements/issues/1743
    let element_ref: NodeRef<Div> = create_node_ref();
    create_effect(move |_| {
        if let Some(element) = element_ref() {
            let jsinput = JsInput::new(&element);
            on_cleanup(move || jsinput.dispose());
        }
    });

    let id = if id.is_empty() { None } else { Some(id) };

    view! {
        <div ref=element_ref class="relative mb-3">
            <input
                type=input_type.map(InputType::html_attrib)
                class=class
                placeholder=label.clone()
                aria-label=label.clone()
                id=id.clone()
                disabled=disabled
                readonly=readonly
                prop:value=value
                on:input=move |ev| {
                    on_change(event_target_value(&ev));
                }
            />
            <label
                for=id
                class="pointer-events-none absolute left-3 top-0 mb-0 max-w-[90%] origin-[0_0] truncate pt-[0.37rem] leading-[1.6] text-neutral-500 transition-all duration-200 ease-out peer-focus:-translate-y-[0.9rem] peer-focus:scale-[0.8] peer-focus:text-primary peer-data-[te-input-state-active]:-translate-y-[0.9rem] peer-data-[te-input-state-active]:scale-[0.8] motion-reduce:transition-none dark:text-neutral-200 dark:peer-focus:text-primary"
            >
                {label}
            </label>
        </div>
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = te, js_name = Input)]
    type JsInput;

    // TODO Input constructor can take some options, see https://tailwind-elements.com/docs/standard/forms/inputs/#docsTabsAPI
    #[wasm_bindgen(constructor, js_namespace = te, js_class = Input, final)]
    fn new(e: &HtmlDivElement) -> JsInput;

    #[wasm_bindgen(method, js_namespace = te, js_class = Input, final)]
    fn dispose(this: &JsInput);
}
