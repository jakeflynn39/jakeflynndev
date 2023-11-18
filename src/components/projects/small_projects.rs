use leptos::*;
use web_sys::{MouseEvent, TouchEvent, HtmlElement};
use wasm_bindgen::JsCast;
use crate::{ProjectInfo, utils::get_document};

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
            image: None,
            color: None,
        },
        ProjectInfo {
            name: "EV Betting".to_string(),
            description: r#"Inspired by OddsJam, built a tool to parse tens of thousands of different lines 
                offered by sportsbooks to calculate positive expected value bets in just a few seconds."#.to_string(),
            short_description: None,
            link: "https://github.com/jakeflynn39/oddsjam-api-plus-ev-calculator".to_string(),
            image: None,
            color: None,
        },
        ProjectInfo {
            name: "Perfect Pitch".to_string(),
            description: r#"Immaculate Grid-style game. Built on SvelteKit. User guesses the song where intersecting 
                artists both performed. Uses the restrictive Spotify API, which does not allow users who are 
                not given explicit "developer access" by me. If you would like to try it, email me, and I 
                would be happy to give you access."#.to_string(),
            short_description: None,
            link: "https://github.com/jakeflynn39/perfect-pitch".to_string(),
            image: None,
            color: None,
        },
        ProjectInfo {
            name: "Griffy Sharps".to_string(),
            description: r#"Made a way for my friend Grif to look at and evaluate his previous
                bets using closing line value to simulate his wins and losses, as well as predict
                betting profitability in the future. Uses Python, Pandas, NumPy, and Matplotlib"#.to_string(),
            short_description: None,
            link: "https://github.com/jakeflynn39/griffy-sharps".to_string(),
            image: None,
            color: None,
        },
        ProjectInfo {
            name: "This Website!".to_string(),
            description: r#"Created this website using WASM, by way of Leptos and Rust. Definitely an overkill
                using Rust to make a simple website, but I had fun doing it and learning how to use a new 
                framework"#.to_string(),
            short_description: None,
            link: "https://github.com/jakeflynn39/jakeflynndev".to_string(),
            image: None,
            color: None,
        },
        
    ];

    let onmousedown = move|e: MouseEvent| {
        let binding: web_sys::EventTarget = e.target().expect("should have a target");
        let target: Option<&HtmlElement> = binding.dyn_ref::<HtmlElement>();

        if let Some(target) = target {
            let small_project: Result<Option<web_sys::Element>, wasm_bindgen::JsValue> = target.closest(".small-project");

            if let Ok(Some(small_project)) = small_project {
                small_project.class_list().add_1("clicked").unwrap();
            }
        }
    };

    let ontouchstart = move|e: TouchEvent| {
        let binding: web_sys::EventTarget = e.target().expect("should have a target");
        let target: Option<&HtmlElement> = binding.dyn_ref::<HtmlElement>();

        let document = get_document();
        let small_projects = document.get_elements_by_class_name("small-project");

        for i in 0..small_projects.length() {
            let small_project = small_projects.item(i).unwrap();
            small_project.class_list().remove_1("clicked").unwrap();
        }

        if let Some(target) = target {
            let small_project: Result<Option<web_sys::Element>, wasm_bindgen::JsValue> = target.closest(".small-project");

            if let Ok(Some(small_project)) = small_project {
                small_project.class_list().add_1("clicked").unwrap();
            }
        }
    };

    let onmouseleave = move|e: MouseEvent| {
        let binding: web_sys::EventTarget = e.target().expect("should have a target");
        let target: Option<&HtmlElement> = binding.dyn_ref::<HtmlElement>();

        if let Some(target) = target {
            let small_project: Result<Option<web_sys::Element>, wasm_bindgen::JsValue> = target.closest(".small-project");

            if let Ok(Some(small_project)) = small_project {
                small_project.class_list().remove_1("clicked").unwrap();
            }
        }
    };

    view! {
        <div class="small-contact">
            <div class="small-projects-container">
            { 
                small_projects
                    .iter()
                    .map(|project| {
                    view! {
                        <div class="small-project" on:mousedown=onmousedown on:touchstart=ontouchstart on:mouseleave=onmouseleave>
                            <div class="full small">
                                <div class="title">
                                    <h2>{ &project.name }</h2>
                                </div>
                                <a href={&project.link} class="link" target="_blank">
                                    <i class="gg-link" />
                                </a>
                            </div>
                            <p>{ &project.description }</p>
                        </div>
                    }
                }).collect_view()
            }
            </div>
            <section class="contact" id="contact">
                <h1>{ "Let's Connect!" }</h1>
                <h2>{ "Open to new opportunites" }</h2>
                <div class="image">
                    <img src="/static/images/jake.png" alt="portrait of JAke Flynn"/>
                </div>
                <div class="clicker">
                    <div class="icon">
                        <a href="https://github.com/jakeflynn39" class="gh" target="_blank">
                            <img src="/static/images/github-mark-white.svg" alt="Github icon" />
                        </a>
                    </div>
                    <a href="/static/JAKE FLYNN WORK SWE RESUME.pdf" class="resume" download="Jake Flynn Resume">
                        { "Resume" }
                    </a>
                </div>
            </section>
        </div>
    }
}