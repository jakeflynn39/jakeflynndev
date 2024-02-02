use leptos::*;

#[component]
pub fn About() -> impl IntoView {
    let about_me = r#"Full stack developer. Nanoscale heat transfer machine learning researcher. 
        Sports analytics enthusiast. Hooper. Skateboarder. Gym-goer. Extremely amateur golfer. Boilermaker."#;

    view! {
        <section class="about-me" id="about">
            <div class="about-section">
                <p>{ about_me }</p>
            </div>
        </section>
    }

}
