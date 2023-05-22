use leptos::*;



//Let's create a progree bar compenent that we can reuse!
#[component]
fn ProgressBar(
    cx: Scope,
    #[prop(into)] progress:Signal<i32> // deriving the prop(into) method allows you to pass any signal, readsignal, or memo into the component
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

    let double_count = Signal::derive(cx, move || count() * 2); // In order to take this into a prop, must use Signal::derive which requires the scope [cx]
    let count_squared = Signal::derive(cx, move || count() * count()); // this is called a derived signal, a closure that accesses a prior established signal


    view! { cx,
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            class=("red", move || i32::abs(count.get()) % 2 == 1)
        >
            "Increase count"
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
        </button>
        <h3>
        "Count squared = "
        {count_squared}
        </h3>
        <ProgressBar progress=count/>
        <ProgressBar progress=double_count/>
        <ProgressBar progress=count_squared/>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}