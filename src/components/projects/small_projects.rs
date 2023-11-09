use leptos::*;
use crate::ProjectInfo;

#[component]
pub fn SmallProjects() -> impl IntoView {
    let small_projects: [ProjectInfo; 5] = [
        ProjectInfo {
            name: "Daily Tweets".to_string(),
            description: r#"Created a bot that tweets at my friends that uses characteristics about them
                using characteristics about them I gave them every morning that uses the Twitter 
                and OpenAI APIs, as well as running on AWS so I do not have to worry about posting
                them every day."#.to_string(),
            link: "/project/daily-tweets".to_string(),
            image: None,
        },
        ProjectInfo {
            name: "EV Betting".to_string(),
            description: r#"Inspired by OddsJam, built a bot to go through different lines offered
                by sportsbooks to calculate positive expected value bets. My first project in
                Rust."#.to_string(),
            link: "/project/ev-betting".to_string(),
            image: None,
        },
        ProjectInfo {
            name: "Perfect Pitch".to_string(),
            description: r#"Inspired by Sports Reference's Immaculate Grid, created a game to guess
                song where artists both performed on. Used the restrictive Spotify API which does 
                not let not users who are not given "developer access" by me play the game. If you 
                would like to play, email me and I would be happy to give you access."#.to_string(),
            link: "/project/perfect-pitch".to_string(),
            image: None,
        },
        ProjectInfo {
            name: "Griffy Sharps".to_string(),
            description: r#"Made a way for my friend Grif to look at and evaluate his previous
                bets using closing line value to simulate his wins and losses, as well as predict
                betting profitiblity in the future."#.to_string(),
            link: "/project/griffy-sharps".to_string(),
            image: None,
        },
        ProjectInfo {
            name: "This Website".to_string(),
            description: r#"Created this website using WASM, by way of Leptos and Rust."#.to_string(),
            link: "/project/this-website".to_string(),
            image: None,
        },
    ];

    view! {
        <div class="small-projects">
        { 
            small_projects.iter().map(|project| {
                view! {
                    <div class="small-project">
                        <h2>{ &project.name }</h2>
                        <p>{ &project.description }</p>
                        <a href={project.link.clone()}>{ "click here to go to project" }</a>
                        { if let Some(image) = &project.image {
                            view! {
                                <>
                                    <div class="card-image">
                                        <img src={image.to_string()} />
                                    </div>
                                </>
                            }.into_view()
                        } else {
                            view! {}.into_view()
                        }}
                    </div>
                }
            }).collect_view()
        }
        </div>
    }
}