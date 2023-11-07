mod models;

use chrono::TimeZone;
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

    let themes = vec!["gruvbox", "light-mode", "dark-mode", "warm", "cool"];
    let theme_index = use_state(|| 0);

    let next_theme = {
        let theme_index = theme_index.clone();
        let themes = themes.clone();
        Callback::from(move |_| {
            let next_index = (*theme_index + 1) % themes.len();
            theme_index.set(next_index);

            // print the current theme
            web_sys::console::log_1(&format!("theme: {}", themes[next_index]).into());
        })
    };

    let prev_theme = {
        let theme_index = theme_index.clone();
        let themes = themes.clone();
        Callback::from(move |_| {
            let prev_index = if *theme_index == 0 {
                themes.len() - 1
            } else {
                *theme_index - 1
            };
            theme_index.set(prev_index);

            // print the current theme
            web_sys::console::log_1(&format!("theme: {}", themes[prev_index]).into());
        })
    };

    html! {
        <div class={classes!("weight-app", themes[*theme_index])}>
            <div class="top-bar">
                <button class="prev-button" onclick={prev_theme}>{ "Previous Theme" }</button>
                <span class={"theme-label"}>{ format!("Current Theme: {}", themes[*theme_index]) }</span>
                <button class="next-button" onclick={next_theme}>{ "Next Theme" }</button>
                <button class="fetch-button" onclick={Callback::from(move |_| fetch_weights.emit(()))}>{ "Fetch Weights" }</button>
                <button class="create-button" onclick={create_weight}>{ "Create Weight" }</button>
            </div>
            <div class="weight-list">
                { for weights.iter().map(|weight| {
                    let delete_callback = make_delete_callback(weight.weight_id);
                    let edit_callback = make_edit_callback(weight.weight_id);
                    let local_time = chrono::Local.from_utc_datetime(&weight.measured_at);
                    let formatted_time = local_time.format("%Y-%m-%d %H:%M").to_string();
                    let weight_in_lbs = weight.weight_kg * 2.20462;
                    html! {
                        <div class="weight-item">
                            <div class="weight-details">
                                // <p class="weight-id">{ format!("ID: {}", weight.weight_id) }</p>
                                // <p class="weight-kg">{ format!("{:.1} kg", weight.weight_kg) }</p>
                                <p class="weight-lbs">{ format!("{:.1} lbs", weight_in_lbs) }</p>
                                <p class="weight-date">{ format!("{}", formatted_time) }</p>
                            </div>
                            <div class="weight-actions">
                                <button class="delete-button" onclick={delete_callback}>{ "Delete" }</button>
                                <button class="edit-button" onclick={edit_callback}>{ "Edit" }</button>
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
