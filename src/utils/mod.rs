use wasm_bindgen::UnwrapThrowExt;
use web_sys::window;
use crate::ProjectRoutes;

pub fn set_title(title: &str) {
    let document = window().expect_throw("no global window exists").document().expect_throw("no global document exists");

    document.set_title(title);
}

pub fn get_document() -> web_sys::Document {
    window().expect_throw("no global window exists").document().expect_throw("no global document exists")
}

pub fn create_element(tag: &str, class: &str, id: &str) -> web_sys::Element {
    let document = get_document();

    let body = document.body().expect_throw("no body found");
    let element = document.create_element(tag).expect_throw("could not create element");
    element.set_class_name(class);
    element.set_id(id);
    body.append_child(&element).expect_throw("could not append child");

    element
}

pub async fn sleep(i: i32) {
    let sleep = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, i as i32)
            .unwrap();
    });
    wasm_bindgen_futures::JsFuture::from(sleep).await.unwrap();
}

pub fn project_url_to_paramter(url: &str) -> Option<ProjectRoutes> {
    match url {
        "hoops-forecast" => Some(ProjectRoutes::HoopsForecast),
        "shot-quality" => Some(ProjectRoutes::ShotQuality),
        "research" => Some(ProjectRoutes::Research),
        "concussion-detection" => Some(ProjectRoutes::ConcussionDetection),
        "daily-tweets" => Some(ProjectRoutes::DailyTweets),
        "ev-betting" => Some(ProjectRoutes::EvBetting),
        "perfect-pitch" => Some(ProjectRoutes::PerfectPitch),
        "griffy-sharps" => Some(ProjectRoutes::GriffySharps),
        "this-website" => Some(ProjectRoutes::ThisWebsite),
        _ => None,
    }
}

pub fn parameter_to_project_url(project: &ProjectRoutes) -> &str {
    match project {
        ProjectRoutes::HoopsForecast => "hoops-forecast",
        ProjectRoutes::ShotQuality => "shot-quality",
        ProjectRoutes::Research => "research",
        ProjectRoutes::ConcussionDetection => "concussion-detection",
        ProjectRoutes::DailyTweets => "daily-tweets",
        ProjectRoutes::EvBetting => "ev-betting",
        ProjectRoutes::PerfectPitch => "perfect-pitch",
        ProjectRoutes::GriffySharps => "griffy-sharps",
        ProjectRoutes::ThisWebsite => "this-website",
        ProjectRoutes::Error => "error",
    }
}