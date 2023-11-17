use leptos::*;
use crate::components::logo::jf_logo::JFLogo;

#[component]
pub fn header() -> impl IntoView {
    view! {
        <header>
            <div class="header-container">
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
