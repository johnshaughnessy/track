mod models;

use gloo_net::http::Request;
use models::CreateWeightPayload;
use models::Weight;
use rand::Rng;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let weights = use_state(|| vec![]);

    let fetch_weights = {
        let weights = weights.clone();
        Callback::from(move |_| {
            let weights = weights.clone();
            spawn_local(async move {
                let fetched_weights: Vec<Weight> =
                    Request::get("http://localhost:8080/api/weights")
                        .send()
                        .await
                        .expect("Failed to fetch weights")
                        .json()
                        .await
                        .expect("Failed to deserialize weights");
                weights.set(fetched_weights);
            });
        })
    };

    {
        let fetch_weights = fetch_weights.clone();
        use_effect_with((), move |_| {
            fetch_weights.emit(());
            || ()
        });
    }

    let create_weight = {
        let fetch_weights = fetch_weights.clone();
        Callback::from(move |_| {
            let fetch_weights = fetch_weights.clone();
            spawn_local(async move {
                let mut rng = rand::thread_rng();
                let weight_kg = rng.gen_range(50.0..100.0);
                let new_weight = CreateWeightPayload {
                    measured_at: chrono::Utc::now().naive_utc(),
                    weight_kg,
                };
                let _ = Request::post("http://localhost:8080/api/weights")
                    .json(&new_weight)
                    .expect("Failed to serialize new weight")
                    .send()
                    .await
                    .map(|_| {
                        fetch_weights.emit(()); // Refetch weights after create
                    });
            });
        })
    };

    let make_delete_callback = {
        let fetch_weights = fetch_weights.clone();
        move |weight_id| {
            let fetch_weights = fetch_weights.clone();
            Callback::from(move |_| {
                let fetch_weights = fetch_weights.clone();
                spawn_local(async move {
                    let _ = Request::delete(&format!(
                        "http://localhost:8080/api/weights/{}",
                        weight_id
                    ))
                    .send()
                    .await
                    .map(|_| {
                        fetch_weights.emit(());
                    })
                    .map_err(|err| {
                        web_sys::console::error_1(
                            &format!("Failed to delete weight: {:?}", err).into(),
                        );
                    });
                });
            })
        }
    };

    let make_edit_callback = {
        let fetch_weights = fetch_weights.clone();
        move |weight_id| {
            let fetch_weights = fetch_weights.clone();
            Callback::from(move |_| {
                // Log "TODO" to the console
                web_sys::console::log_1(&"TODO".into());
                // Also log the weight_id
                web_sys::console::log_1(&format!("weight_id: {}", weight_id).into());
                fetch_weights.emit(());
            })
        }
    };

    html! {
        <div>
            <button onclick={Callback::from(move |_| fetch_weights.emit(()))}>{ "Fetch Weights" }</button>
            <button onclick={create_weight}>{ "Create Weight" }</button>
            <div>
            {
                for weights.iter().map(|weight| {
                    let delete_callback = make_delete_callback(weight.weight_id);
                    let edit_callback = make_edit_callback(weight.weight_id);

                    html! {
                        <div>
                            <p>{ format!("weight_id: {}", weight.weight_id) }</p>
                            <p>{ format!("measured_at: {}", weight.measured_at) }</p>
                            <p>{ format!("weight_kg: {}", weight.weight_kg) }</p>
                            <button onclick={delete_callback}>{ "Delete" }</button>
                            <button onclick={edit_callback}>{ "Edit" }</button>
                        </div>
                    }
                })
            }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
