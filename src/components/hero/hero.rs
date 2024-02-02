use leptos::*;
use rand::{distributions::Alphanumeric, Rng};
use web_sys::MouseEvent;
use crate::utils::{get_document, sleep};

#[component]
pub fn Hero() -> impl IntoView {
    let skills: [Skills; 18] = [
        Skills {
            name: "Javascript".to_string(),
            x: 70,
            y: 17,
            size: 60,
            opacity: 1.0,
        },
        Skills {
            name: "SvelteKit".to_string(),
            x: 0,
            y: 0,
            size: 50,
            opacity: 1.0,
        },
        Skills {
            name: "NextJs".to_string(),
            x: 75,
            y: 83,
            size: 50,
            opacity: 1.0,
        },
        Skills {
            name: "Rust".to_string(),
            x: 60,
            y: 28,
            size: 30,
            opacity: 0.75,
        },
        Skills {
            name: "Python".to_string(),
            x: 8,
            y: 33,
            size: 40,
            opacity: 0.9,
        },
        Skills {
            name: "Typescript".to_string(),
            x: 2,
            y: 92,
            size: 40,
            opacity: 0.9,
        },
        Skills {
            name: "React".to_string(),
            x: 37,
            y: 83,
            size: 50,
            opacity: 0.75,
        },
        Skills {
            name: "Leptos".to_string(),
            x: 37,
            y: 0,
            size: 25,
            opacity: 0.6,
        },
        Skills {
            name: "AWS".to_string(),
            x: 6,
            y: 63,
            size: 25,
            opacity: 0.6,
        },
        Skills {
            name: "Machine Learning".to_string(),
            x: 43,
            y: 110,
            size: 20,
            opacity: 0.6,
        },
        Skills {
            name: "SQL".to_string(),
            x: 61,
            y: 93,
            size: 40,
            opacity: 0.9,
        },
        Skills {
            name: "CatBoost".to_string(),
            x: 72,
            y: 1,
            size: 20,
            opacity: 0.5,
        },
        Skills {
            name: "XGBoost".to_string(),
            x: 61,
            y: 60,
            size: 20,
            opacity: 0.5,
        },
        Skills {
            name: "NodeJs".to_string(),
            x: 37,
            y: 40,
            size: 50,
            opacity: 1.0,
        },
        Skills {
            name: "HTML".to_string(),
            x: 51,
            y: 13,
            size: 30,
            opacity: 1.0,
        },
        Skills {
            name: "CSS".to_string(),
            x: 84,
            y: 50,
            size: 40,
            opacity: 1.0,
        },
        Skills {
            name: "Yew".to_string(),
            x: 24,
            y: 20,
            size: 30,
            opacity: 0.75,
        },
        Skills {
            name: "WebAssembly".to_string(),
            x: 19,
            y: 67,
            size: 30,
            opacity: 0.75,
        },
    ];

    let mut skills_sorted: [Skills; 18] = skills.clone();
    skills_sorted.sort_by(|a, b| a.y.cmp(&b.y));

    let base_definition: String = "Full Stack Web Developer".to_string();
    let base_length: usize = base_definition.len();
    let base_definition_clone = base_definition.clone();
    let random_definition: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(base_length)
        .map(char::from)
        .collect::<String>();

    let onmousemove = move|_: MouseEvent| {
        let document: web_sys::Document = get_document();
        if let Some(skills) = document.get_element_by_id("skills") {
            skills.set_inner_html(&rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(base_length)
                .map(char::from)
                .collect::<String>());
        }
    };

    let onmouseleave = move|_: MouseEvent| {
        let document: web_sys::Document = get_document();
        if let Some(skills) = document.get_element_by_id("skills") {
            skills.set_inner_html(&base_definition);
        }
    };
    
    async fn set_text_on_mount(final_text: &str) {
        let document: web_sys::Document = get_document();
        let skills_element: web_sys::Element = document
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
            sleep(25).await;

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
            <div class="skills-scatter">
                { skills_sorted.iter().map(|skill| {
                    view! {
                        <div 
                            class="skill"
                            style=format!(
                                "--x: {}%; --y: {}%; --size: {}px; --opacity: {};", 
                                skill.x, 
                                skill.y,
                                skill.size,
                                skill.opacity
                            )
                        >
                            <div>{ &skill.name }</div>
                        </div>
                    }
                    }).collect_view()
            }
            </div>
        </div>
    }
}

#[derive(Clone)]
struct Skills {
    name: String,
    x: i32,
    y: i32,
    size: i32,
    opacity: f32,
}
