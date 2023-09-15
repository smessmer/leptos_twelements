use leptos::{html::Nav, *};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use web_sys::HtmlElement;

// TODO Sidenav can so so much more, see https://tailwind-elements.com/docs/standard/navigation/sidenav

#[component]
pub fn Sidenav(#[prop(into)] content_id: String, children: Children) -> impl IntoView {
    // TODO This explicit initialization is a workaround for https://github.com/mdbootstrap/Tailwind-Elements/issues/1743
    let element_ref: NodeRef<Nav> = create_node_ref();
    create_effect(move |_| {
        if let Some(element) = element_ref() {
            let options = JsSidenavOptions {
                hidden: false,
                mode: "side".to_string(),
                content: format!("#{}", content_id),
            };
            let jssidenav =
                JsSidenav::new(&element, serde_wasm_bindgen::to_value(&options).unwrap());
            on_cleanup(move || jssidenav.dispose());
        }
    });

    view! {
        <nav
            ref=element_ref
            class="absolute left-0 top-0 z-[1035] h-screen w-60 -translate-x-full overflow-hidden bg-white shadow-[0_4px_12px_0_rgba(0,0,0,0.07),_0_2px_4px_rgba(0,0,0,0.05)] data-[te-sidenav-hidden='false']:translate-x-0 dark:bg-zinc-800"
            // TODO Not sure why this additional "data-te-sidenav-hidden" is needed, the JavaScript should initialize it correctly.
            data-te-sidenav-hidden="false"
            >
            <ul class="relative m-0 list-none px-[0.2rem]" data-te-sidenav-menu-ref>
                {children()}
            </ul>
        </nav>
    }
}

#[component]
pub fn SidenavItem(#[prop(into)] href: MaybeSignal<String>, children: Children) -> impl IntoView {
    view! {
        <li class="relative">
            <a
                class="flex h-12 cursor-pointer items-center truncate rounded-[5px] px-6 py-4 text-[0.875rem] text-gray-600 outline-none transition duration-300 ease-linear hover:bg-slate-50 hover:text-inherit hover:outline-none focus:bg-slate-50 focus:text-inherit focus:outline-none active:bg-slate-50 active:text-inherit active:outline-none data-[te-sidenav-state-active]:text-inherit data-[te-sidenav-state-focus]:outline-none motion-reduce:transition-none dark:text-gray-300 dark:hover:bg-white/10 dark:focus:bg-white/10 dark:active:bg-white/10"
                href=href
                data-te-sidenav-link-ref>
                {children()}
            </a>
        </li>
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = te, js_name = Sidenav)]
    type JsSidenav;

    #[wasm_bindgen(constructor, js_namespace = te, js_class = Sidenav, final)]
    fn new(e: &HtmlElement, options: JsValue) -> JsSidenav;

    #[wasm_bindgen(method, js_namespace = te, js_class = Sidenav, final)]
    fn dispose(this: &JsSidenav);
}

#[derive(Serialize, Deserialize)]
struct JsSidenavOptions {
    #[serde(rename = "sidenavHidden")]
    hidden: bool,

    #[serde(rename = "sidenavMode")]
    mode: String,

    #[serde(rename = "sidenavContent")]
    content: String,
    // TODO There are more options, see https://tailwind-elements.com/docs/standard/navigation/sidenav/#docsTabsAPI
}
