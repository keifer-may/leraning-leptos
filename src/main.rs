use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    let double_count = move || count() * 2; // this is called a derived signal, a closure that accesses a prior established signal

    view! { cx,
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            class=("red", move || i32::abs(count.get()) % 2 == 1)
        >
            "Increase count"
            // {move || count.get()}
        </button>
        <h2>
            "Count = "
            {move || count.get()}
        </h2>
        <button
            on:click=move |_| {
                set_count.update(|n| *n -= 1);
            }
        >
            "Decrease count"
            // {move || count.get()}
        </button>
        <h3>
        "Count squared = "
        {move || count.get() * count.get() }
        </h3>
        <progress
        max="50"
        // signals are functions, so this <=> `move || count.get()`
        value=double_count
        />   
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}