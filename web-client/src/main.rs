use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let weights = use_state(|| vec![]);

    let onclick = {
        let counter = counter.clone();
        move |_| {
            counter.set(*counter + 1);
        }
    };

    let fetch_weights = {
        let weights = weights.clone();
        Callback::from(move |_| {
            let weights = weights.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let fetched_weights: Vec<Weight> =
                    Request::get("http://localhost:8080/api/weights")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                weights.set(fetched_weights);
            });
        })
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <button onclick={fetch_weights}>{ "Fetch Weights" }</button>
            <p>{ *counter }</p>
            <div>
                { for (*weights).iter().map(|weight| html! {
                    <p>{ format!("Weight: {}", weight.weight_kg) }</p>
                })}
            </div>
        </div>
    }
}

#[derive(Deserialize, Debug)]
struct Weight {
    weight_kg: f64,
}

fn main() {
    yew::Renderer::<App>::new().render();
}
