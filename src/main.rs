use leptos::*;



//Let's create a progree bar compenent that we can reuse!
#[component]
fn ProgressBar(
    cx: Scope,
    #[prop(default = 100)]
    max: u16,
    #[prop(into)]
    progress:Signal<i32> // deriving the prop(into) method allows you to pass any signal, readsignal, or memo into the component
) -> impl IntoView {
    view! { cx,
        <progress
            max=max
            // hmm... where will we get this from?
            value=progress
        />
    }
}

#[component]
fn StaticList(
    cx: Scope,
    #[prop(default = 5)]
    length: usize,
) -> impl IntoView {
    // create a static list of N signals
    let counters = (1..=length).map(|idx| create_signal(cx, idx));

    // each item manages a reactive view
    // but the list itself will never change
    let counter_buttons = counters
    .map(|(count, set_count)| {
        view! { cx,
            <li>
                <button
                    on:click=move |_| set_count.update(|n| *n += 1)
                >
                    {count}
                </button>
            </li>
        }
    })
    .collect_view(cx);

    view! {
        cx,
        <ul>{counter_buttons}</ul>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    let double_count = Signal::derive(cx, move || count() * 2); // In order to take this into a prop, must use Signal::derive which requires the scope [cx]
    let count_squared = Signal::derive(cx, move || count() * count()); // this is called a derived signal, a closure that accesses a prior established signal

    let my_static_values = vec![0,1,2,3];

    

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
        <h3>"Let's discuss and show some examples of iteration in our model"</h3>
        <p>"Here we can show some of our static values we've put into a vector: " {my_static_values.clone()}</p>
        <p>"Why don't we iterate over them: "</p>
        <ul>
            {my_static_values.into_iter()
            .map(|n| view! {cx, <li>{n}</li>})
            .collect_view(cx)}
        </ul>
        <p>"And here is a list of buttons created based on a given static number passed into the function: "</p>
        <StaticList length=6/>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}