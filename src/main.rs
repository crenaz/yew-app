use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 2;
            counter.set(value);
        }
    };

    html! {
        <div>
            <h1>{"This is a Yew app built with Trunk"}</h1>
            <p>{"Just for clicks"}</p>
            <button {onclick}>{ "+2" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
