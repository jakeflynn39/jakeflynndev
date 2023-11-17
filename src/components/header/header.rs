use leptos::*;
use leptos_use::on_click_outside;
use crate::components::logo::jf_logo::JFLogo;

#[component]
pub fn header() -> impl IntoView {
    let dropdown = create_node_ref::<html::Div>();
    let (menu_open, set_menu_open) = create_signal(false);
    let toggle_menu = move |_| set_menu_open(!menu_open.get());

    let _ = on_click_outside(dropdown, move |_| set_menu_open(false));

    view! {
        <header>
            <div class="header-container">
                <div class="filler" />
                <div class="logo-container">
                    <a href="/" class="logo">
                        <JFLogo />
                    </a>
                </div>
                <div class="menu-container" node_ref=dropdown>
                    <div class=move || format!("menu {}", if menu_open() { "" } else { "hidden" })>
                        <a href="#about" on:click=toggle_menu>
                            <div class="header-menu-item">
                                {"About Me"}
                            </div>
                        </a>
                        <a href="#projects" on:click=toggle_menu>
                            <div class="header-menu-item">
                                {"Projects"}
                            </div>
                        </a>
                        <a href="#contact" on:click=toggle_menu>
                            <div class="header-menu-item">
                                {"Contact"}
                            </div>
                        </a>
                    </div>
                    <button
                        class="hamburger"
                        on:click=toggle_menu
                        aria-label="Expand navigation menu"
                    >
                        <div class=move || format!("hamburger-icon {}", if menu_open() { "open" } else { "" })>
                        <span />
                        <span />
                        <span />
                        <span />
                        </div>
                    </button>
                </div>
            </div>
        </header>
    }
}
