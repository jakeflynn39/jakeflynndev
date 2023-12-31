use leptos::*;
use leptos_router::*;
use leptos_meta::*;
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
pub mod utils;
use components::header::header::Header;
use components::hero::hero::Hero;
use components::about::about::About;
use components::projects::big_projects::BigProjects;
use components::projects::small_projects::SmallProjects;
use components::footer::footer::Footer;

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <main>
            <div class="page-container">
                <Header />  
                <Router>
                    <Routes>
                        <Route path="/" view=Home/>
                        <Route path="/*any" view=Error/>
                    </Routes>
                </Router>
            </div>
            <Footer />
        </main>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <Title text="Jake Flynn, Software Developer"/>
        <Meta name="description" content="Jake Flynn's personal website" />
        <Hero />
        <About />
        <BigProjects />
        <SmallProjects />
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
    pub image: Option<String>,
    pub color: Option<Color>,
}

#[derive(Clone)]
pub struct Color {
    pub hue: f64,
    pub saturation: f64,
    pub lightness: f64,
}