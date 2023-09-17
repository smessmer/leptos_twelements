use leptos::*;

// TODO Add non-basic spinners, see https://tailwind-elements.com/docs/standard/components/spinners/

/// The size of a Spinner.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SpinnerSize {
    /// Small Spinner
    Small,
    /// Medium Spinner
    Medium,
    /// Large Spinner
    Large,
}

impl SpinnerSize {
    fn class(self) -> &'static str {
        match self {
            SpinnerSize::Small => "h-4 w-4",
            SpinnerSize::Medium => "h-8 w-8",
            SpinnerSize::Large => "h-12 w-12",
        }
    }
}

/// A Spinner component.
///
/// See [Tailwind Elements: Spinners](https://tailwind-elements.com/docs/standard/components/spinners/)
#[component]
pub fn Spinner(#[prop(default=SpinnerSize::Medium)] size: SpinnerSize) -> impl IntoView {
    let class = format!(
        "inline-block animate-spin rounded-full border-4 border-solid border-current border-r-transparent align-[-0.125em] motion-reduce:animate-[spin_1.5s_linear_infinite] {}",
        size.class()
    );
    view! {
    <div
        class=class
        role="status"
    >
        <span
        class="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]"
        >Loading...</span
        >
    </div>}
}
