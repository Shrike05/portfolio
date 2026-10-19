use dioxus::prelude::*;

#[component]
pub fn TeachingPage() -> Element {
    rsx! {
        div{
            class: "main_vertical",
            h1 { "Shrike the Bike" }
            div {
                class: "Card Flow",
                display: "flex",
                flex_direction: "column",
                align_items: "Center",
                gap: "10px",

                Card{title: "Flow Matching", desc: "Distributional approximation"}
                Card{title: "Blockchain", desc: "Trustless exchange"}
                Card{title: "I2P Project", desc: "Invisible Internet"}
            }
        }
    }
}

#[component]
fn Card(title: String, desc: String) -> Element {
    rsx! {
        div{
            class: "main body",
            border: "1px solid var(--color-border)",
            width: "80vw",
            padding: "10px",
            color: "var(--color-text)",

            h2{"{title}"}
            //Image&Description
            div{
                color: "var(--color-text)",
                "{desc}"
            }
        }
    }
}
