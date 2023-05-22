use leptos::*;



//Let's create a progree bar compenent that we can reuse!
#[component]
fn ProgressBar(
    cx: Scope,
    progress: ReadSignal<i32>
) -> impl IntoView {
    view! { cx,
        <progress
            max="50"
            // hmm... where will we get this from?
            value=progress
        />
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    let double_count = Signal::derive(cx, move || count() * 2); // this is called a derived signal, a closure that accesses a prior established signal
    let second_count = leptos::ReadSignal::<i32>::from(double_count);
    // let second_count =;

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
        {double_count }
        </h3>
        <ProgressBar progress=count/>
        <ProgressBar progress=second_count/>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}