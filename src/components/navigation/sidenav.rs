use leptos::{html::Nav, *};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlElement;

// TODO Sidenav can so so much more, see https://tailwind-elements.com/docs/standard/navigation/sidenav

#[component]
pub fn Sidenav(#[prop(into)] content_id: String, children: Children) -> impl IntoView {
    // TODO This explicit initialization is a workaround for https://github.com/mdbootstrap/Tailwind-Elements/issues/1743
    let element_ref: NodeRef<Nav> = create_node_ref();
    create_effect(move |_| {
        if let Some(element) = element_ref() {
            leptos_tailwind_elements_init_nav(&element);
        }
    });

    view! {
        <nav
            class="fixed left-0 top-0 z-[1035] h-screen w-60 -translate-x-full overflow-hidden bg-white shadow-[0_4px_12px_0_rgba(0,0,0,0.07),_0_2px_4px_rgba(0,0,0,0.05)] data-[te-sidenav-hidden='false']:translate-x-0 dark:bg-zinc-800"
            data-te-sidenav-init
            data-te-sidenav-hidden="false"
            data-te-sidenav-mode="side"
            data-te-sidenav-content=format!("#{content_id}")>
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

// TODO Is this initialization really necessary? Other Tailwind Elements seem to use `new`
//      instead of `getInstance`, so maybe it is not necessary?
#[wasm_bindgen(
    inline_js = "export function leptos_tailwind_elements_init_nav(e) {te.Sidenav.getInstance(e);}"
)]
extern "C" {
    fn leptos_tailwind_elements_init_nav(e: &HtmlElement);
}
