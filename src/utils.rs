//! Utility functions and extension traits for working with signals in Leptos.

use leptos::{html::ElementDescriptor, *};

/// Extension trait adding new methods to build views.
pub trait HtmlElementAttributeExt {
    /// This can be used to create a valueless attribute, e.g. `disabled` or `checked`.
    ///
    /// Unreactive Example:
    /// -----------------
    /// ```
    /// let is_disabled = false;
    /// button()
    ///    .attr_valueless("disabled", is_diabled)
    /// ```
    ///
    /// Reactive Example:
    /// -----------------
    /// ```
    /// let is_disabled = create_signal(cx, false);
    /// button()
    ///     .attr_valueless("disabled", is_disabled)
    /// ```
    fn attr_valueless(
        self,
        name: impl Into<Oco<'static, str>>,
        value: impl Into<MaybeSignal<bool>>,
    ) -> Self;

    /// This can be used to create a boolean attribute, e.g. `attr="true"` or `attr="false"`.
    fn attr_bool(
        self,
        name: impl Into<Oco<'static, str>>,
        value: impl Into<MaybeSignal<bool>>,
    ) -> Self;
}

impl<El: ElementDescriptor + 'static> HtmlElementAttributeExt for HtmlElement<El> {
    fn attr_valueless(
        self,
        name: impl Into<Oco<'static, str>>,
        value: impl Into<MaybeSignal<bool>>,
    ) -> Self {
        let value = value.into();
        self.attr(name, move || if value() { Some("") } else { None })
    }

    fn attr_bool(
        self,
        name: impl Into<Oco<'static, str>>,
        value: impl Into<MaybeSignal<bool>>,
    ) -> Self {
        let value = value.into();
        self.attr(name, move || if value() { "true" } else { "false" })
    }
}

/// Extension trait adding new methods to anything implementing [SignalWith] (e.g. [Signal], [ReadSignal], and [RwSignal]).
pub trait SignalExt<T: 'static> {
    /// This function allows creating a `Signal` that is derived from another `Signal`.
    /// In the dynamic case, the mapping function will be evaluated on each access. This is not a memoized value.
    fn map<U: 'static>(&self, fun: impl Fn(&T) -> U + 'static) -> Signal<U>;
}

impl<S> SignalExt<S::Value> for S
where
    S: SignalWith + Clone + 'static,
    S::Value: Clone + 'static,
{
    fn map<U: 'static>(&self, fun: impl Fn(&S::Value) -> U + 'static) -> Signal<U> {
        let s = self.clone();
        (move || s.with(&fun)).into_signal()
    }
}

/// Extension trait adding new methods to [MaybeSignal].
pub trait MaybeSignalExt<T: 'static> {
    /// This function allows creating a `MaybeSignal` that is derived from another `MaybeSignal`.
    /// If the original `MaybeSignal` is `Static`, the new `MaybeSignal` will be `Static` as well.
    /// If the original `MaybeSignal` is `Dynamic`, the new `MaybeSignal` will be `Dynamic` as well.
    /// In the dynamic case, the mapping function will be evaluated on each access. This is not a memoized value.
    fn map<U: 'static>(&self, fun: impl Fn(&T) -> U + 'static) -> MaybeSignal<U>;
}

impl<T: Clone + 'static> MaybeSignalExt<T> for MaybeSignal<T> {
    fn map<U: 'static>(&self, fun: impl Fn(&T) -> U + 'static) -> MaybeSignal<U> {
        match self {
            MaybeSignal::Static(v) => MaybeSignal::Static(fun(v)),
            MaybeSignal::Dynamic(s) => MaybeSignal::Dynamic(s.map(fun)),
        }
    }
}

/// Extension trait adding new methods to [`Signal<bool>`].
pub trait SignalBoolExt {
    /// Inverts a [`Signal<bool>`] so that it is true when the original signal is false and vice versa.
    fn not(&self) -> Signal<bool>;
    /// ORs a [`Signal<bool>`] with another [`Signal<bool>`] so that the output is true when either of the inputs is true.
    fn or(&self, other: impl Into<Signal<bool>>) -> Signal<bool>;
}

impl<S: SignalGet<Value = bool> + Clone + 'static> SignalBoolExt for S {
    fn not(&self) -> Signal<bool> {
        let s = self.clone();
        (move || !s.get()).into_signal()
    }

    fn or(&self, other: impl Into<Signal<bool>>) -> Signal<bool> {
        let s = self.clone();
        let other = other.into();
        (move || s.get() || other.get()).into_signal()
    }
}

/// Extension trait adding new methods to `MaybeSignal<bool>`.
pub trait MaybeSignalBoolExt {
    /// Inverts a [`MaybeSignal<bool>`] so that it is true when the original signal is false and vice versa.
    fn not(&self) -> Self;
}

impl MaybeSignalBoolExt for MaybeSignal<bool> {
    fn not(&self) -> Self {
        match self {
            MaybeSignal::Static(v) => MaybeSignal::Static(!v),
            MaybeSignal::Dynamic(s) => MaybeSignal::Dynamic(s.not()),
        }
    }
}
