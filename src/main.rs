use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Header {}
        DogApp { breed: "corgi" }
        Footer {}
    }
}

#[component]
fn DogApp(props: DogAppProps) -> Element {
    tracing::info!("Rendered with breed: {breed}");

    todo!()
}

#[derive(Props, PartialEq, Clone)]
struct DogAppProps {
    breed: String,
}
