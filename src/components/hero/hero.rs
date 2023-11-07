use leptos::*;
use rand::{distributions::Alphanumeric, Rng};
use web_sys::MouseEvent;
use crate::utils::{get_document, sleep};

#[component]
pub fn hero() -> impl IntoView {
    let base_definition: String = "Full Stack Web Developer".to_string();
    let base_length = base_definition.len();
    let base_definition_clone = base_definition.clone();
    let random_definition: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(base_length)
        .map(char::from)
        .collect::<String>();

    let onmousemove: Callback<MouseEvent> = Callback::from(move|_: MouseEvent| {
        let document: web_sys::Document = get_document();
        if let Some(skills) = document.get_element_by_id("skills") {
            skills.set_inner_html(&rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(base_length)
                .map(char::from)
                .collect::<String>());
        }
    });

    let onmouseleave: Callback<MouseEvent> = Callback::from(move|_: MouseEvent| {
        let document: web_sys::Document = get_document();
        if let Some(skills) = document.get_element_by_id("skills") {
            skills.set_inner_html(&base_definition);
        }
    });
    
    async fn set_text_on_mount(final_text: &str) {
        let document = get_document();
        let skills_element = document
            .get_element_by_id("skills")
            .unwrap();
    
        let mut rng = rand::thread_rng();
        let init_random_string: String = (0..(final_text.len() - 1))
            .map(|_| rng.sample(&Alphanumeric) as char)
            .collect();
    
        skills_element.set_inner_html(init_random_string.as_str());
        sleep(50).await;
        for (i, _) in final_text.chars().enumerate() {
            let i_as_32 = i as i32;
            sleep(23).await;

            let random_string: String = (0..(final_text.len()) - 1)
                .map(|_| rng.sample(&Alphanumeric) as char)
                .collect();

            let current_text: String = format!("{}{}", &final_text[0..=i_as_32 as usize], &random_string[i_as_32 as usize..]);
            skills_element.set_inner_html(current_text.as_str());
        }
    }


    create_effect(move |_| {
        let cloned = base_definition_clone.clone();
        let async_closure = async move {
            set_text_on_mount(&cloned).await;
        };
        wasm_bindgen_futures::spawn_local(async_closure);
    });

    view! {
        <div 
            class="hero"
        >
            <div 
                class="hero-text"     
                on:mousemove=onmousemove
                on:mouseleave=onmouseleave
            >
                <h1>{ "Jake Flynn" }</h1>
                <h3 id="skills">{ &random_definition }</h3>
            </div>
        </div>
    }
}
