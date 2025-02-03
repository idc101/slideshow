use yew::prelude::*;
use yew_hooks::use_interval;

#[function_component]
fn App() -> Html {
    let time_string = use_state(|| "".to_string());

    {
        let time_string = time_string.clone();
        use_interval(
            move || {
                time_string.set(chrono::Local::now().format("%H:%M:%S").to_string());
            },
            1000,
        )
    }

    html! {
        <div id="imageContainer">
            <div id="clock">{ <std::string::String as Clone>::clone(&*time_string) }</div>
            <img src="/image" />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
