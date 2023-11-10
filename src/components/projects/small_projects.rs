use leptos::*;
use crate::ProjectInfo;

#[component]
pub fn SmallProjects() -> impl IntoView {
    let small_projects: [ProjectInfo; 5] = [
        ProjectInfo {
            name: "Daily Tweets".to_string(),
            description: r#"Created a bot that tweets at my friends using characteristics about them 
                every morning. Uses the Twitter and OpenAI APIs, and runs on AWS with a cron job 
                so it is completely automated."#.to_string(),
            short_description: None,
            link: "https://github.com/jakeflynn39/".to_string(),
            new_tab: true,
            image: None,
        },
        ProjectInfo {
            name: "EV Betting".to_string(),
            description: r#"Inspired by OddsJam, built a tool to go parse thousands of different lines 
                offered by sportsbooks to calculate positive expected value bets. My first project in
                Rust."#.to_string(),
            short_description: None,
            link: "https://github.com/jakeflynn39/".to_string(),
            new_tab: true,
            image: None,
        },
        ProjectInfo {
            name: "Perfect Pitch".to_string(),
            description: r#"Immaculate Grid style game. Built on SvelteKit. Player guess song where intersecting 
                artists both performed on. Uses the restrictive Spotify API which does not let not users who are 
                not given explicit "developer access" by me. If you would like to try it, email me and I 
                would be happy to give you access."#.to_string(),
            short_description: None,
            link: "https://github.com/jakeflynn39/".to_string(),
            new_tab: true,
            image: None,
        },
        ProjectInfo {
            name: "Griffy Sharps".to_string(),
            description: r#"Made a way for my friend Grif to look at and evaluate his previous
                bets using closing line value to simulate his wins and losses, as well as predict
                betting profitiblity in the future. Uses Python, Pandas, Numpy, and Matplotlib"#.to_string(),
            short_description: None,
            link: "https://github.com/jakeflynn39/".to_string(),
            new_tab: true,
            image: None,
        },
        ProjectInfo {
            name: "This Website!".to_string(),
            description: r#"Created this website using WASM, by way of Leptos and Rust. Definitely an overkill
                using Rust to make a simple website, but I had fun doing it and learning how to use a new 
                framework"#.to_string(),
            short_description: None,
            link: "https://github.com/jakeflynn39/".to_string(),
            new_tab: true,
            image: None,
        },
    ];

    view! {
        <div class="small-projects">
            <div class="small-projects-container">
            { 
                small_projects.iter().map(|project| {
                    view! {
                        <div class="small-project">
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
        </div>
    }
}