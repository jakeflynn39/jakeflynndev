use leptos::*;
use web_sys::{MouseEvent, HtmlElement};
use wasm_bindgen::JsCast;
use crate::ProjectInfo;

#[component]
pub fn SmallProjects() -> impl IntoView {
    let small_projects: [ProjectInfo; 5] = [
        ProjectInfo {
            name: "Daily Tweets".to_string(),
            description: r#"Created a bot that tweets at my friends using characteristics about them 
                every morning. Uses the Twitter and OpenAI APIs and runs on AWS with a cron job 
                so it is completely automated."#.to_string(),
            short_description: None,
            link: "https://github.com/jakeflynn39/daily-messages/".to_string(),
            new_tab: true,
            image: None,
        },
        ProjectInfo {
            name: "EV Betting".to_string(),
            description: r#"Inspired by OddsJam, built a tool to parse tens of thousands of different lines 
                offered by sportsbooks to calculate positive expected value bets in just a few seconds."#.to_string(),
            short_description: None,
            link: "https://github.com/jakeflynn39/oddsjam-api-plus-ev-calculator".to_string(),
            new_tab: true,
            image: None,
        },
        ProjectInfo {
            name: "Perfect Pitch".to_string(),
            description: r#"Immaculate Grid-style game. Built on SvelteKit. User guesses the song where intersecting 
                artists both performed. Uses the restrictive Spotify API, which does not allow users who are 
                not given explicit "developer access" by me. If you would like to try it, email me, and I 
                would be happy to give you access."#.to_string(),
            short_description: None,
            link: "https://github.com/jakeflynn39/perfect-pitch".to_string(),
            new_tab: true,
            image: None,
        },
        ProjectInfo {
            name: "Griffy Sharps".to_string(),
            description: r#"Made a way for my friend Grif to look at and evaluate his previous
                bets using closing line value to simulate his wins and losses, as well as predict
                betting profitability in the future. Uses Python, Pandas, NumPy, and Matplotlib"#.to_string(),
            short_description: None,
            link: "https://github.com/jakeflynn39/griffy-sharps".to_string(),
            new_tab: true,
            image: None,
        },
        ProjectInfo {
            name: "This Website!".to_string(),
            description: r#"Created this website using WASM, by way of Leptos and Rust. Definitely an overkill
                using Rust to make a simple website, but I had fun doing it and learning how to use a new 
                framework"#.to_string(),
            short_description: None,
            link: "https://github.com/jakeflynn39/jakeflynndev".to_string(),
            new_tab: true,
            image: None,
        },
        
    ];

    let onclick: Callback<MouseEvent> = Callback::from(move|e: MouseEvent| {
        let binding: web_sys::EventTarget = e.target().expect("should have a target");
        let target: Option<&HtmlElement> = binding.dyn_ref::<HtmlElement>();

        if let Some(target) = target {
            let small_project: Result<Option<web_sys::Element>, wasm_bindgen::JsValue> = target.closest(".small-project");

            if let Ok(Some(small_project)) = small_project {
                small_project.class_list().add_1("clicked").unwrap();
            }
        }
    });

    let onmouseleave: Callback<MouseEvent> = Callback::from(move|e: MouseEvent| {
        let binding: web_sys::EventTarget = e.target().expect("should have a target");
        let target: Option<&HtmlElement> = binding.dyn_ref::<HtmlElement>();

        if let Some(target) = target {
            let small_project: Result<Option<web_sys::Element>, wasm_bindgen::JsValue> = target.closest(".small-project");

            if let Ok(Some(small_project)) = small_project {
                small_project.class_list().remove_1("clicked").unwrap();
            }
        }
    });

    view! {
        <section class="small-contact" id="contact">
            <div class="small-projects-container">
            { 
                small_projects.iter().map(|project| {
                    view! {
                        <div class="small-project" on:click=onclick on:mouseleave=onmouseleave>
                            <div class="full small">
                                <div class="title">
                                    <h2>{ &project.name }</h2>
                                </div>
                                <a href={&project.link} class="link" target={if project.new_tab { "_blank" } else { "_self" }}>
                                    <i class="gg-link" />
                                </a>
                            </div>
                            <p>{ &project.description }</p>
                        </div>
                    }
                }).collect_view()
            }
            </div>
            <div class="contact">
                <h1>{ "Contact Me!" }</h1>
                <h2>{ "Open to new opportunites" }</h2>
                <div class="image">
                    <img src="/static/images/jake.png" alt="portrait of JAke Flynn"/>
                </div>
                <div class="clicker">
                    <div class="icon">
                        <a href="https://github.com/jakeflynn39" target="_blank">
                            <img src="/static/images/github-mark-white.svg" alt="Github icon" />
                        </a>
                    </div>
                    <button>
                        { "My Resume" }
                    </button>
                </div>
            </div>
        </section>
    }
}