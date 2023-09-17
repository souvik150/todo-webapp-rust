use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
         <div class="min-h-screen flex items-center justify-center bg-gray-100">
            <div class="max-w-md space-y-4 p-4 bg-white shadow-lg rounded-lg">
                <button class="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 focus:outline-none" 
                {onclick}>
                    { "+1" }
                </button>
                <p class="text-3xl font-bold text-center">{ *counter }</p>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}