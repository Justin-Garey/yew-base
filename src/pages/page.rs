use crate::app::routes::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Page)]
pub fn apod() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <div>
            <button class="underline text-sm p-3 rounded-full w-auto m-3 p-1" {onclick}>{ "Back" }</button>
            <div class="flex items-center">
                <div class="container w-2/3 shadow-2xl rounded-xl py-3">
                    <div class="flex flex-col">
                        <h1 class="font-bold text-4xl self-center mb-5">{ "You made it to another page" }</h1>
                    </div>
                </div>
            </div>
        </div>
    }
}
