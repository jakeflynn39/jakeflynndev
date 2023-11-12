use leptos::*;
// use web_sys::{HtmlElement, MouseEvent};
// use wasm_bindgen::JsCast;
use crate::components::logo::jf_logo::JFLogo;
// use crate::utils::get_document;

#[component]
pub fn header() -> impl IntoView {
    // let onclick: Callback<MouseEvent> = Callback::from(move|_| {
    //     let document = get_document();
    //     let body = document.body().unwrap();
    //     let body: HtmlElement = body.dyn_into::<HtmlElement>().unwrap();
    //     if body.class_name() == "light" {
    //         body.class_list().remove_1("light").unwrap();
    //     } else {
    //         body.set_class_name("light");
    //     }
    // });

    view! {
        <header>
            <div class="header-container">
                // <div class="toggle">
                //     <input type="checkbox" id="toggle" on:click=onclick/>
                //     <label for="toggle"></label>
                // </div>
                <div class="filler" />
                <div class="logo-container">
                    <a href="/" class="logo">
                        <JFLogo />
                    </a>
                </div>
                <div class="menu-container">
                    <div class="menu">
                        <a href="#about">{"About Me"}</a>
                        <a href="#projects">{"Projects"}</a>
                        <a href="#contact">{"Contact"}</a>
                    </div>
                </div>
            </div>
        </header>
    }
}
