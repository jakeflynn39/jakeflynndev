use leptos::*;
use leptos_use::*;
use web_sys::{PointerEvent, WheelEvent};
use wasm_bindgen::JsCast;
use crate::{ProjectInfo, Color};


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
            image: Some("https://us.canvasartrocks.com/cdn/shop/products/Basketball_court_Wall_Mural_Wallpaper_a_1400x.jpg?v=1571715105".to_string()),
            color: Some(Color {hue: 165.0, saturation: 82.86, lightness: 51.37}),
        },
        ProjectInfo {
            name: "Shot Quality".to_string(),
            description: r#"Freelance full stack NextJs developer. Some data visualization and MySQL when needed. Initiated pages 
                with massive amounts of data and do not need rapid updates to be statically rendered on the server to increase
                page loading times and SEO, amongst other endless Jira tickets"#.to_string(),
            short_description: Some(r#"Freelance"#.to_string()),
            link: "https://shotqualitybets.com/".to_string(),
            image: Some("https://www.sportsbusinessjournal.com/-/media/Images/Daily/2023/03/17/SBJ-Tech/shot-quality.ashx".to_string()),
            color: Some(Color {hue: 291.34, saturation: 95.9, lightness: 61.76}),
        },
        ProjectInfo {
            name: "Heat Transfer Research".to_string(),
            description: r#"Utilized neural networks and thermophysical properties of materials to model atomic-level heat transfer, 
                contributing to the search for radiative cooling materials aimed at mitigating global warming. Proudly part 
                of the Guinness World Record-winning team for creating the whitest and coolest (literally) paint."#.to_string(),
            short_description: Some(r#"ML Research"#.to_string()),
            link: "https://www.purdue.edu/newsroom/releases/2023/Q1/purdues-worlds-whitest-paint-wins-2023-sxsw-innovation-award.html".to_string(),
            image: Some("https://www.purdue.edu/uns/images/2021/ruan-xiulin-portraitLO.jpg".to_string()),
            color: Some(Color {hue: 338.69, saturation: 100.0, lightness: 48.04}),
        },
        ProjectInfo {
            name: "Concussion Detection".to_string(),
            description: r#"Designed and constructed a comprehensive solution for detecting concussions in football players, 
                integrating both software and hardware. Utilized force sensors and BLE technology to transmit real-time data. 
                The development included a custom-made PCB board and a user-friendly interface, resulting in a project that 
                achieved a top 5 ranking in our engineering class."#.to_string(),
            short_description: Some(r#"Embedded Systems"#.to_string()),
            link: "/static/senior_design_poster.pdf".to_string(),
            image: Some("https://mattlaw.com/wp-content/uploads/2016/10/traumatic-brain-injury-symptoms.jpg".to_string()),
            color: Some(Color {hue: 97.5, saturation: 83.71, lightness: 69.0}),
        },
    ];

    let big_projects_clone = big_projects.clone();

    let window = use_window();
    let document = use_document();

    let cards_container = create_node_ref::<html::Div>();
    let overlay: NodeRef<html::Div> = create_node_ref::<html::Div>();

    let mut projects_refs: Vec<NodeRef<html::Div>> = vec![];
    for _ in 0..big_projects.len() {
        projects_refs.push(create_node_ref::<html::Div>());
    }

    let set_overlay = move |e: &ev::Event, leaving: bool| {
        let cards = cards_container
            .get()
            .expect("input_ref should be loaded by now");

        let scroll_height = window
            .as_ref()
            .expect("problem getting window")
            .scroll_y()
            .expect("problem getting scroll height");

        let scroll_width = window
            .as_ref()
            .expect("problem getting window")
            .scroll_x()
            .expect("problem getting scroll width");

        let x: f64;
        let y: f64;

        let _ = if let Some(e) = e.dyn_ref::<PointerEvent>() {
            x = f64::from(e.page_x()) - cards.get_bounding_client_rect().x() - scroll_width;
            y = f64::from(e.page_y()) - cards.get_bounding_client_rect().y() - scroll_height;
        } else if let Some(e) = e.dyn_ref::<WheelEvent>() {
            x = f64::from(e.page_x()) - cards.get_bounding_client_rect().x() - scroll_width;
            y = f64::from(e.page_y()) - cards.get_bounding_client_rect().y() - scroll_height;
        } else {
            panic!("event is neither a pointer event nor a wheel event");
        };

        let opacity = if leaving { 0 } else { 1 };

        overlay
            .get()
            .expect("input_ref should be loaded by now")
            .set_attribute("style", &format!("--x: {}px; --y: {}px; --opacity: {}", x, y, opacity))
            .expect("problem");
    };

    let set_overlay_clone = set_overlay.clone();
    let another_clone = set_overlay.clone();

    let apply_overlay_mask = move |e: PointerEvent| {
        set_overlay(&e, false);
    };

    let remove_overlay = move |e: PointerEvent| {
        set_overlay_clone(&e, true);
    };

    let apply_overlay_scroll = move |e: WheelEvent| {
        another_clone(&e, false);
    };

    create_effect(move |_| {
        for project in &big_projects_clone {
            let overlay_card = document
                .as_ref()
                .expect("problem getting document")
                .create_element("div")
                .expect("problem creating the div");

            let color = project.color.as_ref().unwrap();

            overlay_card.set_class_name("card");

            overlay_card.set_attribute("style", &format!(
                "--hue: {}; --saturation: {}%; --lightness: {}%;",
                color.hue, color.saturation, color.lightness
            )).expect("problem setting attribute");

            overlay
                .get()
                .expect("problem getting overlay")
                .append_child(&overlay_card)
                .expect("problem adding child to card");
        }
    });

    view! {
        <section 
            class="big-projects-cards" 
            id="projects" 
            on:pointermove=apply_overlay_mask 
            on:pointerout=remove_overlay 
            on:wheel=apply_overlay_scroll
        >
            <div class="cards">
                <div class="cards-inner" node_ref=cards_container>
                    {big_projects
                        .into_iter()
                        .zip(projects_refs)
                        .map(|(project, project_ref)| view! {
                            <div class="card" node_ref=project_ref>
                                <div class="card-header">
                                    <div class="full">
                                        <div class="title">
                                            <h2>{ &project.name }</h2>
                                        </div>
                                        <a href={&project.link} class="link" target="_blank">
                                            <i class="gg-link" />
                                        </a>
                                    </div>
                                    <div class="short">
                                        { &project.short_description.unwrap_or("".to_string())}
                                    </div>
                                </div>
                                <div class="card-body">
                                    <div class="card-image" style=format!(
                                        "background-image: url({});", 
                                        {&project
                                            .image
                                            .clone()
                                            .unwrap_or("".to_string())}) />
                                    <p>{ &project.description }</p>
                                </div>
                            </div>
                        }).collect_view()
                    }
                </div>
                <div class="overlay cards-inner" node_ref=overlay />
            </div>
        </section>
    }
}