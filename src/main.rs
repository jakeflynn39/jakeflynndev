use leptos::*;
pub mod components {
    pub mod header {
        pub mod header;
    }
    pub mod logo {
        pub mod jf_logo;
    }
}
use components::header::header::Header;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <>
            <div class="page-container">
                <Header />
                <button
                    on:click=move |_| {
                        set_count(3);
                    }
                >
                    "Click me: "
                    {move || count.get()}
                </button>
            </div>
        </>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
