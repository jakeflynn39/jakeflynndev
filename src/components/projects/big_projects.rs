use leptos::*;
use crate::ProjectInfo;


#[component]
pub fn BigProjects() -> impl IntoView {
    let big_projects: [ProjectInfo; 4] = [
        ProjectInfo{
            name: "Hoops Forecast".to_string(),
            description: r#"Full stack NBA player career trajectory website built with SvelteKit. Hosted on AWS, caching done 
                on CloudFlare. Player models done with CatBoost. Collaboration with 2 of my classmates to launch and maintain 
                throughout the NBA season."#.to_string(),
            short_description: Some(r#"Web app"#.to_string()),
            link: "https://hoopsforecast.com".to_string(),
            new_tab: true,
            image: Some("https://us.canvasartrocks.com/cdn/shop/products/Basketball_court_Wall_Mural_Wallpaper_a_1400x.jpg?v=1571715105".to_string()),
        },
        ProjectInfo {
            name: "Shot Quality".to_string(),
            description: r#"Freelance full stack NextJs developer. Some data visualization and MySQL when needed. Initiated pages 
                with massive amounts of data and do not need rapid updates to be statically rendered on the server to increase
                page loading times and SEO, amongst other endless Jira tickets"#.to_string(),
            short_description: Some(r#"Freelance"#.to_string()),
            link: "https://shotqualitybets.com/".to_string(),
            new_tab: true,
            image: Some("https://www.sportsbusinessjournal.com/-/media/Images/Daily/2023/03/17/SBJ-Tech/shot-quality.ashx".to_string()),
        },
        ProjectInfo {
            name: "Heat Transfer Research".to_string(),
            description: r#"Utilized neural networks and thermophysical properties of materials to model atomic-level heat transfer, 
                contributing to the search for radiative cooling materials aimed at mitigating global warming. Proudly part 
                of the Guinness World Record-winning team for creating the whitest and coolest (literally) paint."#.to_string(),
            short_description: Some(r#"ML Research"#.to_string()),
            link: "https://www.purdue.edu/newsroom/releases/2023/Q1/purdues-worlds-whitest-paint-wins-2023-sxsw-innovation-award.html".to_string(),
            new_tab: true,
            image: Some("https://www.purdue.edu/uns/images/2021/ruan-xiulin-portraitLO.jpg".to_string()),
        },
        ProjectInfo {
            name: "Concussion Detection".to_string(),
            description: r#"Designed and constructed a comprehensive solution for detecting concussions in football players, 
                integrating both software and hardware. Utilized force sensors and BLE technology to transmit real-time data. 
                The development included a custom-made PCB board and a user-friendly interface, resulting in a project that 
                achieved a top 5 ranking in our engineering class."#.to_string(),
            short_description: Some(r#"Embedded Systems"#.to_string()),
            link: "/project/concussion-detection".to_string(),
            new_tab: false,
            image: Some("https://mattlaw.com/wp-content/uploads/2016/10/traumatic-brain-injury-symptoms.jpg".to_string()),
        },
    ];

    view! {
        <section class="big-projects-cards" id="projects">
            <div class="big-projects-cards-inner">
                {big_projects.into_iter()
                    .map(|project| view! {
                        <div class="big-projects-card">
                            <div class="card-header">
                                <div class="full">
                                    <div class="title">
                                        <h2>{ &project.name }</h2>
                                    </div>
                                    <a href={&project.link} class="link" target={if project.new_tab { "_blank" } else { "_self" }}>
                                        <i class="gg-link" />
                                    </a>
                                </div>
                                <div class="short">
                                    { &project.short_description.unwrap_or("".to_string())}
                                </div>
                            </div>
                            <div class="card-body">
                                <div class="card-image" style=format!("background-image: url({});", {&project.image.clone().unwrap_or("".to_string())}) />
                                <p>{ &project.description }</p>
                            </div>
                        </div>
                    }).collect_view()
                    
                }
            </div>
        </section>
    }
}