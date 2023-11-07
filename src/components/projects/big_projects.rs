use yew::prelude::*;
use yew_router::prelude::*;
use crate::{ProjectInfo, Route};


#[function_component(BigProjects)]
pub fn big_projects() -> Html {
    let big_projects: [ProjectInfo; 4] = [
        ProjectInfo{
            name: "Hoops Forecast".to_string(),
            description: r#"A web app that uses machine learning to predict player careers."#.to_string(),
            link: Route::HoopsForecast,
            image: Some("https://us.canvasartrocks.com/cdn/shop/products/Basketball_court_Wall_Mural_Wallpaper_a_1400x.jpg?v=1571715105".to_string()),
        },
        ProjectInfo {
            name: "Shot Quality".to_string(),
            description: r#"Work as a freelance full-stack web dev for them."#.to_string(),
            link: Route::ShotQuality,
            image: Some("https://www.sportsbusinessjournal.com/-/media/Images/Daily/2023/03/17/SBJ-Tech/shot-quality.ashx".to_string()),
        },
        ProjectInfo {
            name: "Nancoscale Heat Transfer ML Research".to_string(),
            description: r#"Used machine learning to and thermophysical properties to predict materials 
                with radiative cooling for my undergrad research. Part of the team with the Guinness 
                world record for whitest paint ever."#.to_string(),
            link: Route::UndergradResearch,
            image: Some("https://www.purdue.edu/uns/images/2021/ruan-xiulin-portraitLO.jpg".to_string()),
        },
        ProjectInfo {
            name: "Realtime Wirless Concussion Detection".to_string(),
            description: r#"Designed and built both the software and hardware to detect concussions 
                in football players using force sensors and BLE to transmit the data in real-time. 
                Project finished top 5 in engineering class."#.to_string(),
            link: Route::Home,
            image: Some("https://mattlaw.com/wp-content/uploads/2016/10/traumatic-brain-injury-symptoms.jpg".to_string()),
        },
    ];

    html! {
        <div 
            class="big-projects-cards"
        >
            <
                div class="big-projects-cards-inner"
            >
                { 
                    big_projects.iter().map(|project| {
                        html! {
                            <div class="big-projects-card">
                                <div class="card-header">
                                    <h2>{ &project.name }</h2>
                                    <p>{ &project.description }</p>
                                    <Link<Route> to={project.link.clone()}>{ "click here to go to project" }</Link<Route>>
                                </div>
                                // { if let Some(image) = &project.image {
                                //     html! {
                                //         <>
                                //             <div class="card-image">
                                //                 <img src={image.to_string()} />
                                //             </div>
                                //         </>
                                //     }
                                // } else {
                                //     html! {}
                                // }}
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}