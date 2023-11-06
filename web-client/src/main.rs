use yew::{html, Component, Context, Html};

pub enum Msg {
    Increment,
}

pub struct App {
    value: i64, // This will store the counter value
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button
                    onclick={ctx.link().callback(|_| Msg::Increment)}
                >{ "Increment" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.value += 1;
                true // Indicate that the component should re-render
            }
        }
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
