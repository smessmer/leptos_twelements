use leptos::{
    html::{Div, A},
    *,
};
use std::sync::{Arc, Mutex};
use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen};
use web_sys::HtmlDivElement;

// TODO Modal can do so much more. Make this more flexible, e.g. allow call sites to give us Views not just Strings for the modal sections.

/// A Modal component.
///
/// See [Tailwind Elements: Modal](https://tailwind-elements.com/docs/standard/components/modal/)
pub struct Modal<ContentFn, ContentView>
where
    ContentFn: Clone + Fn(Arc<ModalImpl>) -> ContentView + 'static,
    ContentView: IntoView,
{
    content: ContentFn,
    modal_impl: Arc<ModalImpl>,
    showing: ReadSignal<bool>,
}

impl<ContentFn, ContentView> Modal<ContentFn, ContentView>
where
    ContentFn: Clone + Fn(Arc<ModalImpl>) -> ContentView + 'static,
    ContentView: IntoView,
{
    /// TODO Docs
    // TODO Builder pattern instead of constructor
    pub fn new(content: ContentFn) -> Self {
        let (showing, set_showing) = create_signal(false);
        Self {
            content,
            modal_impl: Arc::new(ModalImpl {
                jsmodal: Mutex::new(None),
                set_showing,
            }),
            showing,
        }
    }

    /// Add this to your page so that the modal view can be shown.
    ///
    /// Example
    /// -------
    /// ```
    /// use leptos::*;
    /// use leptos_twelements::Modal;
    ///
    /// #[component]
    /// pub fn Page() -> impl IntoView {
    ///    let modal = Modal::new();
    ///    view! {
    ///       {modal.view()}
    ///       <Button on_click=move |_| modal.show()>{"Show modal"}</Button>
    ///    }
    /// }
    /// ```
    pub fn view(&self) -> impl IntoView {
        view! {
            <ModalView modal_impl=Arc::clone(&self.modal_impl) content=self.content.clone() />
        }
    }

    /// TODO Docs
    pub fn show(&self) {
        self.modal_impl.show();
    }

    /// TODO Docs
    pub fn showing(&self) -> ReadSignal<bool> {
        self.showing
    }
}

// TODO Better name for ModalImpl
pub struct ModalImpl {
    jsmodal: Mutex<Option<JsModal>>,
    set_showing: WriteSignal<bool>,
}

impl ModalImpl {
    pub fn show(&self) {
        let jsmodal = self.jsmodal.lock().unwrap();
        let Some(jsmodal) = jsmodal.as_ref() else {
            panic!("Tried to show a modal but its view is not added to the page");
        };
        jsmodal.show();
    }

    pub fn hide(&self) {
        let jsmodal = self.jsmodal.lock().unwrap();
        let Some(jsmodal) = jsmodal.as_ref() else {
            panic!("Tried to hide a modal but its view is not added to the page");
        };
        jsmodal.hide();
    }
}

#[component]
fn ModalView<ContentFn, ContentView>(
    modal_impl: Arc<ModalImpl>,
    content: ContentFn,
) -> impl IntoView
where
    ContentFn: Fn(Arc<ModalImpl>) -> ContentView + 'static,
    ContentView: IntoView,
{
    // TODO This explicit initialization is a workaround for https://github.com/mdbootstrap/Tailwind-Elements/issues/1743
    let modal_ref: NodeRef<Div> = create_node_ref();
    let modal_impl_clone = Arc::clone(&modal_impl);
    create_effect(move |_| {
        if let Some(element) = modal_ref() {
            let modal_impl = Arc::clone(&modal_impl_clone);
            *modal_impl.jsmodal.lock().unwrap() = Some(JsModal::new(&element));

            let on_show: Closure<dyn FnMut()> = {
                let modal_impl = Arc::clone(&modal_impl);
                Closure::new(move || {
                    (modal_impl.set_showing)(true);
                })
            };

            let on_hidden: Closure<dyn FnMut()> = {
                let modal_impl = Arc::clone(&modal_impl);
                Closure::new(move || {
                    (modal_impl.set_showing)(false);
                })
            };

            te_modal_add_event_listener(&element, "show.te.modal", &on_show);
            te_modal_add_event_listener(&element, "hidden.te.modal", &on_hidden);

            on_cleanup(move || {
                let jsmodal = modal_impl.jsmodal.lock().unwrap().take();
                if let Some(jsmodal) = jsmodal {
                    jsmodal.dispose();
                }
                std::mem::drop(on_show);
                std::mem::drop(on_hidden);
            });
        }
    });
    provide_context::<Arc<ModalImpl>>(Arc::clone(&modal_impl));

    view! {
        <div
            ref=modal_ref
            class="fixed left-0 top-0 z-[1055] hidden h-full w-full overflow-y-auto overflow-x-hidden outline-none"
            // TODO Set ids through out the modal correctly
            id="exampleModalCenter"
            tabindex="-1"
            aria-labelledby="exampleModalCenterTitle"
            aria-modal="true"
            role="dialog">
            <div
                data-te-modal-dialog-ref
                class="pointer-events-none relative flex min-h-[calc(100%-1rem)] w-auto translate-y-[-50px] items-center opacity-0 transition-all duration-300 ease-in-out min-[576px]:mx-auto min-[576px]:mt-7 min-[576px]:min-h-[calc(100%-3.5rem)] min-[576px]:max-w-[500px]">
                <div
                    class="pointer-events-auto relative flex w-full flex-col rounded-md border-none bg-white bg-clip-padding text-current shadow-lg outline-none dark:bg-neutral-600">
                    {content(modal_impl)}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ModalHeader(children: Children) -> impl IntoView {
    let modal_impl = use_context::<Arc<ModalImpl>>().expect("Expected ModalImpl in context");
    view! {
        <div
            class="flex flex-shrink-0 items-center justify-between rounded-t-md border-b-2 border-neutral-100 border-opacity-100 p-4 dark:border-opacity-50">
            // Modal title
            <h5
                class="text-xl font-medium leading-normal text-neutral-800 dark:text-neutral-200"
                id="exampleModalCenterTitle">
                {children()}
            </h5>
            // Close button
            <CloseButton on:click=move |_| modal_impl.hide() />
        </div>
    }
}

#[component]
pub fn ModalBody(children: Children) -> impl IntoView {
    view! {
        <div class="relative p-4">
            {children()}
        </div>
    }
}

#[component]
pub fn ModalFooter(children: Children) -> impl IntoView {
    view! {
        <div
            class="flex flex-shrink-0 flex-wrap items-center justify-end rounded-b-md border-t-2 border-neutral-100 border-opacity-100 p-4 gap-4 dark:border-opacity-50">
            {children()}
        </div>
    }
}

#[component]
fn CloseButton() -> impl IntoView {
    view! {
        <button
            type="button"
            class="box-content rounded-none border-none hover:no-underline hover:opacity-75 focus:opacity-100 focus:shadow-none focus:outline-none"
            aria-label="Close">
            <CloseButtonIcon />
        </button>
    }
}

#[component]
fn CloseButtonIcon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="h-6 w-6">
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M6 18L18 6M6 6l12 12" />
        </svg>
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = te, js_name = Modal)]
    type JsModal;

    #[wasm_bindgen(constructor, js_namespace = te, js_class = Modal, final)]
    fn new(e: &HtmlDivElement) -> JsModal;

    #[wasm_bindgen(method, js_namespace = te, js_class = Modal, final)]
    fn show(this: &JsModal);

    #[wasm_bindgen(method, js_namespace = te, js_class = Modal, final)]
    fn hide(this: &JsModal);

    #[wasm_bindgen(method, js_namespace = te, js_class = Modal, final)]
    fn dispose(this: &JsModal);
}

#[wasm_bindgen(
    inline_js = "export function te_modal_add_event_listener(modal_html_elem, event_name, callback) { modal_html_elem.addEventListener(event_name, (event) => {
        callback(event.to);
    }); }"
)]
extern "C" {
    #[wasm_bindgen]
    fn te_modal_add_event_listener(
        modal: &web_sys::HtmlElement,
        event_name: &str,
        callback: &Closure<dyn FnMut()>,
    );
}
