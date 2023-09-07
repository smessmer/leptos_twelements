use leptos::*;

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
    cx: Scope,
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
    let id = if id.is_empty() { None } else { Some(id) };

    view! {cx,
        <div class="relative mb-3" data-te-input-wrapper-init>
            <input
                type=input_type.map(cx, InputType::html_attrib)
                class=class
                placeholder=label.clone()
                aria-label=label.clone()
                id=id.clone()
                disabled=disabled
                readonly=readonly
                prop:value=value.clone()
                // Setting data-te-input-state-active is a Workaround for https://github.com/mdbootstrap/Tailwind-Elements/issues/1743
                data-te-input-state-active=move || if value.with(|v| v.is_empty()) { None } else { Some("") }
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
            <InputBorder />
        </div>
    }
}

// TODO The twelements example highlights this border when it's active. We don't do that yet correctly.
// TODO The border is currently visible through the label text instead of hiding behind it.
#[component]
fn InputBorder(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="group flex absolute left-0 top-0 w-full max-w-full h-full text-left pointer-events-none" data-te-input-notch-ref>
            <div
                class="pointer-events-none border border-solid box-border bg-transparent transition-all duration-200 ease-linear motion-reduce:transition-none left-0 top-0 h-full w-2 border-r-0 rounded-l-[0.25rem] group-data-[te-input-focused]:border-r-0 group-data-[te-input-state-active]:border-r-0 border-neutral-300 dark:border-neutral-600 group-data-[te-input-focused]:shadow-[-1px_0_0_#3b71ca,_0_1px_0_0_#3b71ca,_0_-1px_0_0_#3b71ca] group-data-[te-input-focused]:border-primary"
                data-te-input-notch-leading-ref
                style="width: 9px;">
            </div>
            <div
                class="pointer-events-none border border-solid box-border bg-transparent transition-all duration-200 ease-linear motion-reduce:transition-none grow-0 shrink-0 basis-auto w-auto max-w-[calc(100%-1rem)] h-full border-r-0 border-l-0 group-data-[te-input-focused]:border-x-0 group-data-[te-input-state-active]:border-x-0 group-data-[te-input-focused]:border-t group-data-[te-input-state-active]:border-t group-data-[te-input-focused]:border-solid group-data-[te-input-state-active]:border-solid group-data-[te-input-focused]:border-t-transparent group-data-[te-input-state-active]:border-t-transparent border-neutral-300 dark:border-neutral-600 group-data-[te-input-focused]:shadow-[0_1px_0_0_#3b71ca] group-data-[te-input-focused]:border-primary"
                data-te-input-notch-middle-ref
                style="width: 87.2px;">
            </div>
            <div
                class="pointer-events-none border border-solid box-border bg-transparent transition-all duration-200 ease-linear motion-reduce:transition-none grow h-full border-l-0 rounded-r-[0.25rem] group-data-[te-input-focused]:border-l-0 group-data-[te-input-state-active]:border-l-0 border-neutral-300 dark:border-neutral-600 group-data-[te-input-focused]:shadow-[1px_0_0_#3b71ca,_0_-1px_0_0_#3b71ca,_0_1px_0_0_#3b71ca] group-data-[te-input-focused]:border-primary"
                data-te-input-notch-trailing-ref>
            </div>
        </div>
    }
}
