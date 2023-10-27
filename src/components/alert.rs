use leptos::*;

// TODO Add non-basic spinners, see https://tailwind-elements.com/docs/standard/components/spinners/

/// The size of a Spinner.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AlertType {
    /// Primary Alert
    Primary,
    /// Secondary Alert
    Secondary,
    /// Success Alert
    Success,
    /// Danger Alert
    Danger,
    /// Warning Alert
    Warning,
    /// Info Alert
    Info,
    /// Light Alert
    Light,
    /// Dark Alert
    Dark,
}

impl AlertType {
    fn class(&self) -> &'static str {
        match self {
            Self::Primary => "bg-primary-100 text-primary-600",
            Self::Secondary => "bg-secondary-100 text-secondary-800",
            Self::Success => "bg-success-100 text-success-700",
            Self::Danger => "bg-danger-100 text-danger-700",
            Self::Warning => "bg-warning-100 text-warning-800",
            Self::Info => "bg-info-100 text-info-800",
            Self::Light => "bg-neutral-50 text-neutral-600",
            Self::Dark => "bg-neutral-800 text-neutral-50 dark:bg-neutral-900",
        }
    }
}

/// An Alert component.
///
/// See [Tailwind Elements: Alerts](https://tw-elements.com/docs/standard/components/alerts/)
#[component]
pub fn Alert(
    #[prop(default=AlertType::Primary)] alert_type: AlertType,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=move || format!("mb-4 rounded-lg px-6 py-5 text-base {}", alert_type.class())
            role="alert"
        >
            {children()}
        </div>
    }
}

// <span class="mr-2">
//     <svg
//         xmlns="http://www.w3.org/2000/svg"
//         viewBox="0 0 24 24"
//         fill="currentColor"
//         class="h-5 w-5">
//         <path
//         fill-rule="evenodd"
//         d="M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25zm-1.72 6.97a.75.75 0 10-1.06 1.06L10.94 12l-1.72 1.72a.75.75 0 101.06 1.06L12 13.06l1.72 1.72a.75.75 0 101.06-1.06L13.06 12l1.72-1.72a.75.75 0 10-1.06-1.06L12 10.94l-1.72-1.72z"
//         clip-rule="evenodd" />
//     </svg>
// </span>
