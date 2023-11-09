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
use utils::project_url_to_paramter;

#[component]
fn App() -> impl IntoView {
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
    view! {
        <Title text="JAke's World"/>
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
    pub link: String,
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