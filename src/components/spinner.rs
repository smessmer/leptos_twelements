use leptos::*;

// TODO Add non-basic spinners, see https://tailwind-elements.com/docs/standard/components/spinners/

/// A Spinner component.
///
/// See [Tailwind Elements: Spinners](https://tailwind-elements.com/docs/standard/components/spinners/)
#[component]
pub fn Spinner() -> impl IntoView {
    view! {
    <div
        class="inline-block h-4 w-4 animate-spin rounded-full border-4 border-solid border-current border-r-transparent align-[-0.125em] motion-reduce:animate-[spin_1.5s_linear_infinite]"
        role="status"
    >
        <span
        class="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]"
        >Loading...</span
        >
    </div>}
}
