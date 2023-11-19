use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div class="footer-link">
                <div class="icon">
                    <a href="https://github.com/jakeflynn39" target="_blank">
                        <img src="/static/images/github-mark-white.svg" alt="Github icon" />
                    </a>
                </div>
                <div class="icon">
                    <a href="https://www.linkedin.com/in/jake-p-flynn/" target="_blank">
                        <img src="/static/images/linkedin.webp" alt="LinkedIn icon" />
                    </a>
                </div>
            </div>
        </footer>
    }
}