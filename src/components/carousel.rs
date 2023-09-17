use leptos::{html::Div, *};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::HtmlDivElement;

use crate::utils::SignalExt;

// TODO Make more flexible, allow more options

/// A Carousel component
///
/// See [Tailwind Elements: Carousel](https://tailwind-elements.com/docs/standard/components/carousel/)
#[component]
pub fn Carousel(
    // TODO Use Duration instead of f32
    #[prop(into)] interval: MaybeSignal<f32>,
    #[prop(into, default=vec![].into())] images: MaybeSignal<Vec<CarouselImage>>,
    // TODO Auto-assign id
    #[prop(into)] id: String,
) -> impl IntoView {
    // TODO This explicit initialization is a workaround for https://github.com/mdbootstrap/Tailwind-Elements/issues/1743
    // TODO Do we even need to initialize the carousel? We're doing our own logic for the "current slide" anyways.
    let element_ref: NodeRef<Div> = create_node_ref();
    create_effect(move |_| {
        if let Some(element) = element_ref() {
            let options = JsCarouselOptions {
                interval: interval(),
                ride: "carousel".to_string(),
                pause: "hover".to_string(),
            };
            let jscarousel =
                JsCarousel::new(&element, serde_wasm_bindgen::to_value(&options).unwrap());
            on_cleanup(move || jscarousel.dispose());
        }
    });

    // TODO Do we want a memo of `image_with_index` or of `image` to prevent re-computing the memo? PartialEq on the Vec isn't cheap either.
    let images_with_index = create_memo(move |_| {
        images.with(|images| images.iter().cloned().enumerate().collect::<Vec<_>>())
    });

    let id_clone = id.clone();

    // TODO The buttons don't seem to work yet. Why?

    // TODO Put parts of this view (e.g. the buttons) into subcomponents
    view! {
        <div ref=element_ref id=id.clone() class="relative">
            // Carousel Indicators
            <div
                class="absolute bottom-0 left-0 right-0 z-[2] mx-[15%] mb-4 flex list-none justify-center p-0"
                data-te-carousel-indicators
            >
                // TODO Do we actually need `key` or is it enough to use `index` because the elements themselves are reactive anyways?
                <For each=images_with_index key=|(_index, img)| img.key view=move |(index, _img)| view!{
                    <button
                        type="button"
                        data-te-target=format!("#{id}", id=id_clone)
                        data-te-slide-to=index
                        data-te-carousel-active={index == 0} // TODO is this the right way of doing it? Or should we build our own reactive signal for "current image" and apply it to the right one?
                        class="mx-[3px] box-content h-[3px] w-[30px] flex-initial cursor-pointer border-0 border-y-[10px] border-solid border-transparent bg-white bg-clip-padding p-0 -indent-[999px] opacity-50 transition-opacity duration-[600ms] ease-[cubic-bezier(0.25,0.1,0.25,1.0)] motion-reduce:transition-none"
                        aria-current={if index == 0 {Some("true")} else {None}} // TODO is this the right way of doing it? Or should we build our own reactive signal for "current image" and apply it to the right one?
                        aria-label=format!("Image {index}") />
                } />
            </div>

            // Carousel Items
            <div
                class="relative w-full overflow-hidden after:clear-both after:block after:content-['']"
            >
                <For each=images_with_index key=|(_index, img)| img.key view=move |(index, img)| {
                    let mut class = "relative float-left -mr-[100%] w-full transition-transform duration-[600ms] ease-in-out motion-reduce:transition-none".to_string();
                    if index != 0 {
                        class.push_str(" hidden");
                    }
                    view!{
                    <div
                        class=class
                        data-te-carousel-active={index == 0} // TODO is this the right way of doing it? Or should we build our own reactive signal for "current image" and apply it to the right one?
                        data-te-carousel-item
                        style="backface-visibility: hidden"
                    >
                        <img
                            src=img.src
                            class="block w-full"
                            alt=img.alt />
                        <div
                            class="absolute inset-x-[15%] bottom-5 hidden py-5 text-center text-white md:block">
                            <h5 class="text-xl">
                                {img.title}
                            </h5>
                            <p>
                                {img.subtitle}
                            </p>
                        </div>
                    </div>
                }} />
            </div>

            // Carousel controls: prev item
            <button
                class="absolute bottom-0 left-0 top-0 z-[1] flex w-[15%] items-center justify-center border-0 bg-none p-0 text-center text-white opacity-50 transition-opacity duration-150 ease-[cubic-bezier(0.25,0.1,0.25,1.0)] hover:text-white hover:no-underline hover:opacity-90 hover:outline-none focus:text-white focus:no-underline focus:opacity-90 focus:outline-none motion-reduce:transition-none"
                type="button"
                data-te-target=format!("#{id}", id=id)
                data-te-slide="prev"
            >
                <span class="inline-block h-8 w-8">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="h-6 w-6"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M15.75 19.5L8.25 12l7.5-7.5" />
                    </svg>
                </span>
                <span
                    class="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]"
                >
                    {"Previous"}
                </span>
            </button>

            // Carousel controls: next item
            <button
                class="absolute bottom-0 right-0 top-0 z-[1] flex w-[15%] items-center justify-center border-0 bg-none p-0 text-center text-white opacity-50 transition-opacity duration-150 ease-[cubic-bezier(0.25,0.1,0.25,1.0)] hover:text-white hover:no-underline hover:opacity-90 hover:outline-none focus:text-white focus:no-underline focus:opacity-90 focus:outline-none motion-reduce:transition-none"
                type="button"
                data-te-target=format!("#{id}", id=id)
                data-te-slide="next"
            >
                <span class="inline-block h-8 w-8">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="h-6 w-6"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M8.25 4.5l7.5 7.5-7.5 7.5" />
                    </svg>
                </span>
                <span
                    class="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]"
                >
                    {"Next"}
                </span>
            </button>
        </div>
    }
}

/// TODO Docs
#[derive(Clone, PartialEq)]
pub struct CarouselImage {
    /// TODO Docs
    pub key: i32,

    /// TODO Docs
    pub title: String,

    /// TODO Docs
    pub subtitle: String,

    /// TODO Docs
    pub src: String,

    /// TODO Docs
    pub alt: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = te, js_name = Carousel)]
    type JsCarousel;

    // TODO Carousel constructor can take some options, see https://tailwind-elements.com/docs/standard/components/Carousel/#docsTabsAPI
    #[wasm_bindgen(constructor, js_namespace = te, js_class = Carousel, final)]
    fn new(e: &HtmlDivElement, options: JsValue) -> JsCarousel;

    #[wasm_bindgen(method, js_namespace = te, js_class = Carousel, final)]
    fn dispose(this: &JsCarousel);
}

#[derive(Serialize, Deserialize)]
struct JsCarouselOptions {
    // TODO This can be number|boolean. If we set it to `false`, then the carousel doesn't change between pictures.
    interval: f32,

    // TODO can be string|boolean
    ride: String,

    // TODO can be string|boolean
    pause: String,
    // TODO There are more options, see https://tailwind-elements.com/docs/standard/components/carousel/#docsTabsAPI
}
