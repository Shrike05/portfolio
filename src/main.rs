use dioxus::prelude::*;
use landing_page::LandingPage;
use projects::ProjectsPage;
use teaching::TeachingPage;

mod landing_page;
mod projects;
mod teaching;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    //#[layout(SideBar)]
    #[route("/")]
    LandingPage {},
    #[route("/teaching")]
    TeachingPage {},
    #[route("/projects")]
    ProjectsPage {}
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel:"preconnect", href:"https://fonts.googleapis.com" }
        document::Link { rel:"preconnect", href:"https://fonts.gstatic.com" }
        document::Link { rel:"stylesheet", href:"https://fonts.googleapis.com/css2?family=Share+Tech+Mono&display=swap" }
        Router::<Route> {}
    }
}
