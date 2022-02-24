use yew::prelude::*;

#[function_componet]
fn App() -> Html {
    Let counter = use_state(|| 0);
    Let onclick = {
        Let counter = counter.clone();
        move |_| {
            Let value = *couter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1+" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::startapp::<App>();
}
