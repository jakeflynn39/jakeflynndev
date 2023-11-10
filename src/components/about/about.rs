use leptos::*;

#[component]
pub fn About() -> impl IntoView {
    let about_me = r#"Full stack developer. Nanoscale heat transfer machine learning researcher. 
        Sports analytics enthusiast. Hiker. Skateboarder. Gym-goer. Extremely amateur golfer. Boilermaker."#;

    view! {
        <div class="about-me">
            <div class="about-section">
                <p>{ about_me }</p>
            </div>
        </div>
    }

}