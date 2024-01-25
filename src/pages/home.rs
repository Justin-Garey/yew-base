use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::routes::*;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Page));

    html! {
        <div class="h-screen flex items-center">
            <div class="container w-2/3 shadow-2xl rounded-xl py-3 bg-white">
                <div class="flex flex-col">
                    <h1 class="font-bold text-4xl self-center mb-5">{ "Welcome to the Yew-Base!" }</h1>
                    <p class="self-center my-5">{ "Hopefully this provides a good launchpad for creating Yew based applications." }</p>
                    <button class="bg-gray-800 hover:bg-gray-600 text-white font-bold p-3 rounded-full w-auto self-center mt-5" onclick={onclick.clone()}>{ "To another page" }</button>
                </div>
            </div>
        </div>
    }
}
