use leptos::*;
use leptos_router::*;
use crate::{set_title, project_url_to_paramter, ProjectRoutes};

#[component]
pub fn projects() -> impl IntoView {
    let params: Memo<Result<ProjectParams, ParamsError>> = use_params::<ProjectParams>();
    let name: String = {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.name.clone())
                .unwrap()
        })
    };

    let title: &str;

    if let Some(project) = project_url_to_paramter(&name) {
        title = match project {
            ProjectRoutes::HoopsForecast => "Hoops Forecast",
            ProjectRoutes::ShotQuality => "Shot Quality",
            ProjectRoutes::Research => "Research",
            ProjectRoutes::ConcussionDetection => "Concussion Detection",
            ProjectRoutes::DailyTweets => "Daily Tweets",
            ProjectRoutes::EvBetting => "EV Betting",
            ProjectRoutes::PerfectPitch => "Perfect Pitch",
            ProjectRoutes::GriffySharps => "Griffy Sharps",
            ProjectRoutes::ThisWebsite => "This Website",
            ProjectRoutes::Error => "Error",
        };
    } else {
        title = "Error";
    }

    // set_title(title);

    view! {
        <div>
            <h1>{ "Projects" }</h1>
            <h2>{ name }</h2>
            <a href="/">{ "click here to go home" }</a>
        </div>
    }
}

#[derive(Params, PartialEq)]
struct ProjectParams {
    name: String,
}

