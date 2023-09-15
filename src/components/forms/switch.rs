use leptos::*;

/// A Toggle Switch component
///
/// See [Tailwind Elements: Switch](https://tailwind-elements.com/docs/standard/forms/switch/)
#[component]
pub fn Switch<OnChangeFn: Fn(bool) + 'static>(
    /// This signal is read to show the current state of the switch
    /// Whenever this signal changes, the switch is updated.
    /// But note that the switch can also be toggled by the user and if the `on_change` function doesn't
    /// update the `checked` signal, the switch can get out of sync with the signal.
    #[prop(into)]
    checked: MaybeSignal<bool>,
    /// Callback that is called when the switch is toggled.
    on_change: OnChangeFn,
    /// Whether the switch is disabled.
    #[prop(into, default = false.into(),)]
    disabled: MaybeSignal<bool>,
    /// The id of the switch. Useful to associate a label with it.
    #[prop(into, default = "".into())]
    id: String,
) -> impl IntoView {
    let class = move || {
        let mut class = "mr-2 mt-[0.3rem] h-3.5 w-8 appearance-none rounded-[0.4375rem] bg-neutral-300 before:pointer-events-none before:absolute before:h-3.5 before:w-3.5 before:rounded-full before:bg-transparent before:content-[''] after:absolute after:z-[2] after:-mt-[0.1875rem] after:h-5 after:w-5 after:rounded-full after:border-none after:bg-neutral-100 after:shadow-[0_0px_3px_0_rgb(0_0_0_/_7%),_0_2px_2px_0_rgb(0_0_0_/_4%)] after:transition-[background-color_0.2s,transform_0.2s] after:content-[''] checked:bg-primary checked:after:absolute checked:after:z-[2] checked:after:-mt-[3px] checked:after:ml-[1.0625rem] checked:after:h-5 checked:after:w-5 checked:after:rounded-full checked:after:border-none checked:after:bg-primary checked:after:shadow-[0_3px_1px_-2px_rgba(0,0,0,0.2),_0_2px_2px_0_rgba(0,0,0,0.14),_0_1px_5px_0_rgba(0,0,0,0.12)] checked:after:transition-[background-color_0.2s,transform_0.2s] checked:after:content-[''] hover:cursor-pointer focus:outline-none focus:ring-0 focus:before:scale-100 focus:before:opacity-[0.12] focus:before:shadow-[3px_-1px_0px_13px_rgba(0,0,0,0.6)] focus:before:transition-[box-shadow_0.2s,transform_0.2s] focus:after:absolute focus:after:z-[1] focus:after:block focus:after:h-5 focus:after:w-5 focus:after:rounded-full focus:after:content-[''] checked:focus:border-primary checked:focus:bg-primary checked:focus:before:ml-[1.0625rem] checked:focus:before:scale-100 checked:focus:before:shadow-[3px_-1px_0px_13px_#3b71ca] checked:focus:before:transition-[box-shadow_0.2s,transform_0.2s] dark:bg-neutral-600 dark:after:bg-neutral-400 dark:checked:bg-primary dark:checked:after:bg-primary dark:focus:before:shadow-[3px_-1px_0px_13px_rgba(255,255,255,0.4)] dark:checked:focus:before:shadow-[3px_-1px_0px_13px_#3b71ca]".to_string();
        if disabled() {
            class.push_str(" disabled:cursor-default disabled:opacity-60");
        };
        class
    };

    view! {
        <input
            type="checkbox"
            role="switch"
            class=class
            id=if id.is_empty() { None } else { Some(id) }
            disabled=disabled
            prop:checked=checked
            on:input=move |ev| {
                on_change(event_target_checked(&ev));
            }
        />
    }
}
