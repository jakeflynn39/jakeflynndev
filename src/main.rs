use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use leptos_use::*;
use web_sys::{HtmlElement, PointerEvent, console, MouseEvent, TouchEvent};
use wasm_bindgen::JsCast;
pub mod components {
    pub mod header {
        pub mod header;
    }
    pub mod logo {
        pub mod jf_logo;
    }
    pub mod icons {
        pub mod paperclip;
    }
    pub mod hero {
        pub mod hero;
    }
    pub mod about {
        pub mod about;
    }
    pub mod projects {
        pub mod big_projects;
        pub mod small_projects;
    }
    pub mod footer {
        pub mod footer;
    }
}
pub mod routes {
    pub mod projects;
}
pub mod utils;
use components::header::header::Header;
use components::hero::hero::Hero;
use components::about::about::About;
use components::projects::big_projects::BigProjects;
use components::projects::small_projects::SmallProjects;
use components::footer::footer::Footer;
use routes::projects::Projects;
use utils::{project_url_to_paramter, get_document};

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <main>
                <div class="page-container">
                    <Header />
                    <Routes>
                        <Route path="/" view=Home/>
                        <Route path="/project/:name" view=Projects/>
                        <Route path="/*any" view=Error/>
                    </Routes>
                </div>
                <Footer />
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let document = use_document();

    let cards_container = create_node_ref::<html::Div>();
    let overlay = create_node_ref::<html::Div>();

    let apply_overlay_mask = move |e: PointerEvent| {
        let cards = cards_container
            .get()
            .expect("input_ref should be loaded by now");

        let binding: web_sys::EventTarget = e
            .target()
            .expect("should have a target");

        let overlay_el: Option<&HtmlElement> = binding.dyn_ref::<HtmlElement>();

        if let Some(overlay_el) = overlay_el {
            let x = e.page_x() - cards.offset_left();
            let y = e.page_y() - cards.offset_top();
            overlay_el
                .closest(".cards")
                .expect("problem getting closest")
                .expect("problem getting closest")
                .set_attribute("style", &format!("--x: {}; --y: {}; --opacity: 1", x, y))
                .expect("problem");
        }
    };

    create_effect(move |_| {
        let cards = document
            .body()
            .expect("body should be here lowkey")
            .query_selector_all(".card")
            .expect("should be able to query selector all");

        for _ in 0..cards.length() {
            let overlay_card = document
                .as_ref()
                .expect("problem getting document")
                .create_element("div")
                .expect("problem creating the div");

            overlay_card.set_class_name("overlay");
            overlay
                .get()
                .expect("problem getting overlay")
                .append_child(&overlay_card)
                .expect("problem adding child to card");
        }
    });

    view! {
        <Title text="Jake Flynn, Software Developer"/>
        <Meta name="description" content="Jake Flynn's personal website" />
        // <Hero />
        // <About />
        // <BigProjects />
        // <SmallProjects />
        <main class="main flow">
            <h1 class="main__heading">Pricing</h1>
            <div class="main__cards cards" node_ref=cards_container on:pointermove=apply_overlay_mask>
                <div class="cards__inner">
                <div class="cards__card card">
                    <h2 class="card__heading">Basic</h2>
                    <p class="card__price">$9.99</p>
                    <ul role="list" class="card__bullets flow">
                    <li>Access to standard workouts and nutrition plans</li>
                    <li>Email support</li>
                    </ul>
                    <a href="#basic" class="card__cta cta">Get Started</a>
                </div>

                <div class="cards__card card">
                    <h2 class="card__heading">Pro</h2>
                    <p class="card__price">$19.99</p>
                    <ul role="list" class="card__bullets flow">
                    <li>Access to advanced workouts and nutrition plans</li>
                    <li>Priority Email support</li>
                    <li>Exclusive access to live Q&A sessions</li>
                    </ul>
                    <a href="#pro" class="card__cta cta">Upgrade to Pro</a>
                </div>

                <div class="cards__card card">
                    <h2 class="card__heading">Ultimate</h2>
                    <p class="card__price">$29.99</p>
                    <ul role="list" class="card__bullets flow">
                    <li>Access to all premium workouts and nutrition plans</li>
                    <li>24/7 Priority support</li>
                    <li>1-on-1 virtual coaching session every month</li>
                    <li>Exclusive content and early access to new features</li>
                    </ul>
                    <a href="#ultimate" class="card__cta cta">Go Ultimate</a>
                </div>
                </div>
                
                <div class="overlay cards__inner" node_ref=overlay></div>
            </div>
            </main>
    }
}

#[component]
fn Error() -> impl IntoView {
    view! {
        <div>
            <h2>{ "Page not found" }</h2>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[derive(Clone)]
pub struct ProjectInfo {
    pub name: String,
    pub description: String,
    pub short_description: Option<String>,
    pub link: String,
    pub new_tab: bool,
    pub image: Option<String>,
}

pub enum ProjectRoutes {
    HoopsForecast,
    ShotQuality,
    Research,
    ConcussionDetection,
    DailyTweets,
    EvBetting,
    PerfectPitch,
    GriffySharps,
    ThisWebsite,
    Error,
}