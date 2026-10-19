use dioxus::prelude::*;

const CARD_IMG: Asset = asset!("/assets/imgs/bg0.png");

#[component]
pub fn ProjectsPage() -> Element {
    rsx! {
        div{
            class: "main_vertical",
            h1 { "Shrike the Bike" }
            Card{
                img_path: "/assets/imgs/bg0.png",
                name:"Classified Evidence", desc: "An illegal dealing",
                tags: vec!["Classified".to_string(), "Black Market".to_string(), "Kitty".to_string()]
            }
        }
    }
}

#[component]
fn Card(img_path: String, name: String, desc: String, tags: Vec<String>) -> Element {
    rsx! {
        div {
            class: "top_card",
            div{
                class: "inner_card",
                img {
                    class: "card_img",
                    src: img_path
                }
                h3  {
                    margin_top: "10px",
                    class: "card_name",
                    "{name}"
                }
                "{desc}"
                div {
                    class: "tags",
                    for tag in tags{
                        div {
                            class: "card_tag",
                            "{tag}"
                        }
                    }
                }
            }
        }
    }
}
