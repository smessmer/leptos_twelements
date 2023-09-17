use leptos::{html::Div, *};
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::HtmlDivElement;

// TODO Make more flexible, allow more options

/// A Carousel component
///
/// See [Tailwind Elements: Carousel](https://tailwind-elements.com/docs/standard/components/carousel/)
#[component]
pub fn Carousel(
    // TODO Make optional (and don't auto-slide when None). This is how the Tailwind Elements API handles it.
    #[prop(into)] interval: MaybeSignal<Duration>,
    #[prop(into, default=vec![].into())] images: MaybeSignal<Vec<CarouselImage>>,
    // TODO Auto-assign id
    #[prop(into)] id: String,
) -> impl IntoView {
    let carousel_id: Oco<'_, str> = Oco::Owned(id);

    let jscarousel = Arc::new(Mutex::new(None));

    // TODO This explicit initialization is a workaround for https://github.com/mdbootstrap/Tailwind-Elements/issues/1743
    let element_ref: NodeRef<Div> = create_node_ref();
    let jscarousel_clone = Arc::clone(&jscarousel);
    create_effect(move |_| {
        if let Some(element) = element_ref() {
            let options = JsCarouselOptions {
                interval: i32::try_from(interval().as_millis()).expect("duration out of bounds"),
                ride: "carousel".to_string(),
                pause: "hover".to_string(),
            };
            let mut jscarousel_guard = jscarousel_clone.lock().unwrap();
            assert!(jscarousel_guard.is_none(), "Tried to set JsCarousel twice");
            *jscarousel_guard = Some(JsCarousel::new(
                &element,
                serde_wasm_bindgen::to_value(&options).unwrap(),
            ));
            std::mem::drop(jscarousel_guard);
            let jscarousel_clone = Arc::clone(&jscarousel_clone);
            on_cleanup(move || {
                jscarousel_clone
                    .lock()
                    .unwrap()
                    .take()
                    .expect(
                        "Tried to drop a JsCarousel that was already dropped or never initialized",
                    )
                    .dispose()
            });
        }
    });

    view! {
        <div ref=element_ref id=carousel_id.clone() class="relative h-full">
            <CarouselIndicators carousel_id=carousel_id.clone() images=images.clone() jscarousel=Arc::clone(&jscarousel) />
            <CarouselItems images />
            <CarouselPrevNextButton carousel_id=carousel_id.clone() direction=PrevNext::Prev jscarousel=Arc::clone(&jscarousel) />
            <CarouselPrevNextButton carousel_id direction=PrevNext::Next jscarousel=Arc::clone(&jscarousel) />
        </div>
    }
}

#[component]
fn CarouselIndicators(
    carousel_id: Oco<'static, str>,
    images: MaybeSignal<Vec<CarouselImage>>,
    jscarousel: Arc<Mutex<Option<JsCarousel>>>,
) -> impl IntoView {
    let indices = create_memo(move |_| images.with(|images| (0..images.len())));
    view! {
        <div
            class="absolute bottom-0 left-0 right-0 z-[2] mx-[15%] mb-4 flex list-none justify-center p-0"
            data-te-carousel-indicators
        >
            <For each=indices key=|index| *index view=move |index| view!{
                {
                    let jscarousel = Arc::clone(&jscarousel);
                    view! {
                        <button
                            type="button"
                            data-te-target=format!("#{carousel_id}")
                            data-te-slide-to=index
                            on:click=move |_| {
                                jscarousel
                                    .lock()
                                    .unwrap()
                                    .as_ref()
                                    .expect("Carousel not initialized")
                                    .to(index)
                            }
                            data-te-carousel-active={index == 0} // Set the first image to be initially active
                            class="mx-[3px] box-content h-[3px] w-[30px] flex-initial cursor-pointer border-0 border-y-[10px] border-solid border-transparent bg-white bg-clip-padding p-0 -indent-[999px] opacity-50 transition-opacity duration-[600ms] ease-[cubic-bezier(0.25,0.1,0.25,1.0)] motion-reduce:transition-none"
                            aria-current={if index == 0 {Some("true")} else {None}} // Set the first image to be initially active
                            aria-label=format!("Image {index}") />
                    }
                }
            } />
        </div>
    }
}

#[component]
fn CarouselItems(images: MaybeSignal<Vec<CarouselImage>>) -> impl IntoView {
    let images_clone = images.clone();
    let indices = create_memo(move |_| images_clone.with(|images| (0..images.len())));
    view! {
        <div
            class="relative w-full h-full overflow-hidden after:clear-both after:block after:content-['']"
        >
            <For each=indices key=|index| *index view=move |index| {
                // TODO Are all these clone calls needed?
                let images_1 = images.clone();
                let images_2 = images.clone();
                let images_3 = images.clone();
                let images_4 = images.clone();
                let src = move || with!(|images_1| images_1[index].src.clone());
                let alt = move || with!(|images_2| images_2[index].alt.clone());
                let title = move || with!(|images_3| images_3[index].title.clone());
                let subtitle = move || with!(|images_4| images_4[index].subtitle.clone());

                let mut class = "relative h-full float-left -mr-[100%] w-full transition-transform duration-[600ms] ease-in-out motion-reduce:transition-none".to_string();
                if index != 0 {
                    class.push_str(" hidden");
                }
                view!{
                <div
                    class=class
                    data-te-carousel-active={index == 0} // Set the first image to be initially active
                    data-te-carousel-item
                    style="backface-visibility: hidden"
                >
                    <img
                        src=src
                        class="block w-full absolute h-full object-cover"
                        alt=alt />
                    <div
                        class="absolute inset-x-[15%] bottom-5 hidden py-5 text-center text-white md:block">
                        <h5 class="text-xl">
                            {title}
                        </h5>
                        <p>
                            {subtitle}
                        </p>
                    </div>
                </div>
            }} />
        </div>
    }
}

enum PrevNext {
    Prev,
    Next,
}

#[component]
fn CarouselPrevNextButton(
    carousel_id: Oco<'static, str>,
    #[prop(into)] direction: PrevNext,
    jscarousel: Arc<Mutex<Option<JsCarousel>>>,
) -> impl IntoView {
    let position = match direction {
        PrevNext::Prev => "left-0",
        PrevNext::Next => "right-0",
    };
    let slide = match direction {
        PrevNext::Prev => "prev",
        PrevNext::Next => "next",
    };
    let svg_path = match direction {
        PrevNext::Prev => "M15.75 19.5L8.25 12l7.5-7.5",
        PrevNext::Next => "M8.25 4.5l7.5 7.5-7.5 7.5",
    };
    let alt = match direction {
        PrevNext::Prev => "Previous",
        PrevNext::Next => "Next",
    };
    let on_click = match direction {
        PrevNext::Prev => |jscarousel: &JsCarousel| jscarousel.prev(),
        PrevNext::Next => |jscarousel: &JsCarousel| jscarousel.next(),
    };
    let class = format!("absolute bottom-0 {position} top-0 z-[1] flex w-[15%] items-center justify-center border-0 bg-none p-0 text-center text-white opacity-50 transition-opacity duration-150 ease-[cubic-bezier(0.25,0.1,0.25,1.0)] hover:text-white hover:no-underline hover:opacity-90 hover:outline-none focus:text-white focus:no-underline focus:opacity-90 focus:outline-none motion-reduce:transition-none");
    view! {
        <button
            class=class
            type="button"
            on:click=
                move |_| {
                    {
                        leptos_dom::logging::console_log("Clicked prev/next button");
                        on_click(&jscarousel
                            .lock()
                            .unwrap()
                            .as_ref()
                            .expect("Carousel not initialized"));
                        leptos_dom::logging::console_log("Clicked prev/next button...done");
                    }
                    leptos_dom::logging::console_log("Clicked prev/next button...clean");
                }
            data-te-target=format!("#{carousel_id}")
            data-te-slide=slide
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
                        d=svg_path />
                </svg>
            </span>
            <span
                class="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]"
            >
                {alt}
            </span>
        </button>
    }
}

/// An image shown in a carousel
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CarouselImage {
    /// A title string to be shown on top of the image
    pub title: String,

    /// A subtitle string to be shown on top of the image
    pub subtitle: String,

    /// The source URL of the image file
    pub src: String,

    /// An `alt` text for accessibility
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
    fn prev(this: &JsCarousel);

    #[wasm_bindgen(method, js_namespace = te, js_class = Carousel, final)]
    fn next(this: &JsCarousel);

    #[wasm_bindgen(method, js_namespace = te, js_class = Carousel, final)]
    fn to(this: &JsCarousel, index: usize);

    #[wasm_bindgen(method, js_namespace = te, js_class = Carousel, final)]
    fn dispose(this: &JsCarousel);
}

#[derive(Serialize, Deserialize)]
struct JsCarouselOptions {
    // TODO This can be number|boolean. If we set it to `false`, then the carousel doesn't change between pictures.
    interval: i32,

    // TODO can be string|boolean
    ride: String,

    // TODO can be string|boolean
    pause: String,
    // TODO There are more options, see https://tailwind-elements.com/docs/standard/components/carousel/#docsTabsAPI
}
