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
    #[layout(SideBar)]
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

/// Shared navbar component.
#[component]
fn SideBar() -> Element {
    let mut is_open = use_signal(|| true);

    rsx! {
        div {
            display: "flex",
            height: "100vh",
            overflow: "hidden",

            // Sidebar
            div {
                min_width: if is_open() {"5vh"} else {"0px"},
                width: if is_open() {"auto"} else {"45px"},
                height: "100%",
                background_color: "var(--color-bg)",
                color: "var(--color-text)",
                transition: "width 0.2s ease",
                display: "flex",
                flex_direction: "column",
                padding: "10px",
                border_right: "1px solid var(--color-border)",

                // Sidebar Header with Toggle Button
                div {
                    display: "flex",
                    justify_content: "space-between",
                    align_items: "center",
                    border_bottom: "1px solid var(--color-border)",
                    min_height: "32px",

                    button {
                        onclick: move |_| {is_open.toggle(); println!("Pressed sidebar")},
                        background_color: "transparent",
                        border: "none",
                        color: "white",
                        font_size: "24px",
                        cursor: "pointer",
                        padding: "4px 8px",
                        opacity: "1",
                        // Rotate arrow when closed
                        transform: if !is_open() { "rotate(180deg)" } else { "rotate(0deg)" },
                        transition: "transform 0.3s ease",
                        "☰"
                    }
                }

                // Sidebar Content (fades in/out)
                div {
                    overflow: "hidden",
                    white_space: "nowrap",
                    flex: "1",
                    width: "auto",
                    margin_top: "10px",

                    // Navigation items
                    nav {
                        display: "flex",
                        flex_direction: "column",
                        gap: "8px",

                        SideBarItem { name: "🏠 Home", slim_name: "🏠", route: "/", is_open: is_open }
                        SideBarItem { name: "🏫 Teaching", slim_name: "🏫", route: "/teaching", is_open: is_open }
                        SideBarItem { name: "🦀 Projects", slim_name: "🦀", route: "/projects", is_open: is_open }
                    }
                }
            }
            Outlet::<Route> {}
        }
    }
}

#[component]
fn SideBarItem(name: String, slim_name: String, route: String, is_open: Signal<bool>) -> Element {
    rsx! {
        a {
            href: "{route}",
            color: "var(--color-text)",
            text_decoration: "none",
            padding: "10px 12px",
            background_color: "var(--color-bg)",
            border: "1px solid var(--color-border)",
            if is_open() {"{name}"} else {"{slim_name}"}
        }
    }
}
