use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_interval;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/settings")]
    SettingsForm,
}

#[function_component]
fn Home() -> Html {
    let time_string = use_state(|| "".to_string());
    let image_num = use_state(|| 0);

    {
        let time_string = time_string.clone();
        use_interval(
            move || {
                time_string.set(chrono::Local::now().format("%H:%M").to_string());
            },
            1000,
        )
    }

    {
        let image_num = image_num.clone();
        use_interval(
            move || {
                let image_num = image_num.clone();
                spawn_local(async move {
                    let settings: Settings = Request::get(format!("/api/settings").as_str())
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    image_num.set((chrono::Local::now().timestamp() as i32) / settings.interval);
                });
            },
            30000,
        )
    }

    html! {
        <div id="imageContainer">
            <div id="clock">{ <std::string::String as Clone>::clone(&*time_string) }</div>
            <img src={format!("/api/image/{}", *image_num)} />
            <Metadata num={*image_num} />
        </div>
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Settings {
    slideshow: String,
    interval: i32,
}

#[function_component]
fn SettingsForm() -> Html {
    // let items = use_state(|| Vec::new());
    // let name_ref = use_node_ref();
    // let interval_ref = use_node_ref();

    // // Fetch items on component mount
    // {
    //     let items = items.clone();
    //     use_effect_with_deps(
    //         move |_| {
    //             spawn_local(async move {
    //                 let fetched_items: Vec<Item> = Request::get("/items")
    //                     .send()
    //                     .await
    //                     .unwrap()
    //                     .json()
    //                     .await
    //                     .unwrap();
    //                 items.set(fetched_items);
    //             });
    //             || ()
    //         },
    //         (),
    //     );
    // }

    // let onsubmit = {
    //     let items = items.clone();
    //     let name_ref = name_ref.clone();
    //     let interval_ref = interval_ref.clone();

    //     Callback::from(move |e: SubmitEvent| {
    //         e.prevent_default();
    //         let items = items.clone();
    //         let name = name_ref.cast::<HtmlInputElement>().unwrap().value();
    //         let interval = interval_ref
    //             .cast::<HtmlInputElement>()
    //             .unwrap()
    //             .value()
    //             .parse::<i32>()
    //             .unwrap_or(0);

    //         // Create new item
    //         let new_item = Item { name, interval };

    //         spawn_local(async move {
    //             let created_item: Item = Request::post("/items")
    //                 .json(&new_item)
    //                 .unwrap()
    //                 .send()
    //                 .await
    //                 .unwrap()
    //                 .json()
    //                 .await
    //                 .unwrap();

    //             // Update local state
    //             let mut current_items = (*items).clone();
    //             current_items.push(created_item);
    //             items.set(current_items);
    //         });
    //     })
    // };

    html! {
        <h1>{"Hello"}</h1>
        // <div class="container">
        //     <h1>{"Item Manager"}</h1>

        //     // Form
        //     <form onsubmit={onsubmit}>
        //         <div class="form-group">
        //             <label for="name">{"Name:"}</label>
        //             <input
        //                 type="text"
        //                 id="name"
        //                 ref={name_ref.clone()}
        //                 required=true
        //             />
        //         </div>

        //         <div class="form-group">
        //             <label for="interval">{"Interval:"}</label>
        //             <input
        //                 type="number"
        //                 id="interval"
        //                 ref={interval_ref.clone()}
        //                 required=true
        //                 min="1"
        //             />
        //         </div>

        //         <button type="submit">{"Add Item"}</button>
        //     </form>

        //     // Display items
        //     <div class="items-list">
        //         <h2>{"Items"}</h2>
        //         <ul>
        //             {(*items).iter().map(|item| {
        //                 html! {
        //                     <li>
        //                         {format!("Name: {}, Interval: {}", item.name, item.interval)}
        //                     </li>
        //                 }
        //             }).collect::<Html>()}
        //         </ul>
        //     </div>
        // </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct MetadataProps {
    pub num: i32,
}

#[function_component]
fn Metadata(props: &MetadataProps) -> Html {
    let metadata = use_state(|| String::from(""));

    // Fetch items on component mount
    {
        let metadata = metadata.clone();
        let num = props.num.clone();
        use_effect_with(num, move |_| {
            spawn_local(async move {
                let fetched: String = Request::get(format!("/api/image/{}/metadata", num).as_str())
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                metadata.set(fetched);
            });
        });
    }

    html! {
        <div id="metadata">
            {(*metadata.clone()).as_str()}
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::SettingsForm => html! {
            <SettingsForm />
        },
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
