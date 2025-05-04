use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! { "HotDog!" }
}

#[component]
fn DogApp(props: DogAppProps) -> Element {
    todo!()
}

#[derive(Props, PartialEq, Clone)]
struct DogAppProps {
    breed: String,
}
