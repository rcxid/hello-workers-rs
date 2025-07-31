use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component]
pub fn Counter() -> impl IntoView {
    let (value, set_value) = signal(0_i64);
    let reset = move |_| set_value.set(0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    view! {
        <div>
            <p class:text-red-500=move || value.get() % 2 == 1>
                <strong>"响应式: "</strong>
                // you can insert Rust expressions as values in the DOM
                // by wrapping them in curly braces
                // if you pass in a function, it will reactively update
                {move || value.get()}
            </p>
            <p class=("text-green-500", move || value.get() % 2 == 0)>
                <strong>"响应式简洁写法: "</strong>
                // signals are functions, so we can remove the wrapping closure
                {value}
            </p>
            <p>
                <strong>"非响应式: "</strong>
                // NOTE: if you write {count()}, this will *not* be reactive
                // it simply gets the value of count once
                {value.get()}
            </p>
            <button on:click=decrement>"点击减1"</button>
            <button on:click=reset>"数值重置"</button>
            <button on:click=increment>"点击加1"</button>
        </div>
    }
}
