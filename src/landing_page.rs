use dioxus::prelude::*;

const CARD_IMG: Asset = asset!("/assets/imgs/bg0.png");

#[component]
pub fn LandingPage() -> Element {
    rsx! {

        div{
            class: "main_horizontal",
            div{
                background_color: "#0F0C15",
                border: "1px solid",
                border_color: "#8B5CF6",
                "NavBar",
            },
            div {
                background_color: "#0F0C15",
                border: "1px solid",
                border_color: "#8B5CF6",
                "Main Flow"
            }
            div {
                background_color: "#0F0C15",
                border: "1px solid",
                border_color: "#8B5CF6",
                "Other bar"
            }
        }
    }
}
