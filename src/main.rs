use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App)
}

#[component]
fn app() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let html = "<p>This is inner HTML injected.</p>";
    let double_count = move || count() * 2;
    let values = vec![0, 1, 2];
    
    view! {
        <div style="text-align: center" inner_html=html />
        <div style="display: flex; flex-direction: column; justify-content: center; align-items: center">
            <ProgressBar max=10 progress=count />
            <ProgressBar progress=count />
            <ProgressBar max=110 progress=count />
            <ProgressBar max=300 progress=Signal::derive(double_count)/>
            <button
                on:click={move |_| {
                    set_count.update(|n| *n += 10);
                }}          
                style:background-color=move || format!("rgb({}, {}, 205)", count(), 90)
                class:red= move || count() % 2 == 0 
            >
                "Click me: "
                {move || count()}
            </button>
            <br/>
            <p>{values.clone()}</p>
            <ul style="margin-left: -25px">
                {values.into_iter()
                    .map(|n| view! {<li>{n}</li>})
                    .collect_view()
                }
            </ul>
            <br/>
            <DynamicButtons />
            <br/>
            <DynamicList initial_length=5/>
            <br/>
            <DbEntries />
        </div>
    }
}

#[component]
fn progress_bar(
    #[prop(default = 100)]
    max: u16,
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView {
    view! {
        <progress
            max={max}
            value=progress
        />
        <br/>
    }
}

#[component]
fn dynamic_buttons() -> impl IntoView {
    let length = 5;
    let counters = (1..=length).map(|idx| create_signal(idx));
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button on:click=move |_| set_count.update(|n| *n += 1)>
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view();

    view! {
        <ul>{counter_buttons}</ul>
    }
}

#[component]
fn dynamic_list(
    initial_length: usize,
) -> impl IntoView {
    let mut next_counter_id = initial_length;
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();
    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        let sig = create_signal(next_counter_id + 1);
        set_counters.update(move |counters| {
            counters.push((next_counter_id, sig))
        });
        next_counter_id += 1;
    };

    view! {
        <div>
            <button style="margin-left:20px" on:click=add_counter>
                "Add Counter"
            </button>
            <ul>
                <For
                    each=counters
                    key=|counter| counter.0
                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <button
                                    on:click=move |_| set_count.update(|n| *n += 1)
                                >
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters.update(|counters| {
                                            counters.retain(|(counter_id, (signal, _))| {
                                                if counter_id == &id {
                                                    signal.dispose();
                                                }
                                                counter_id != &id
                                            })
                                        });
                                    }
                                >
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
pub fn db_entries() -> impl IntoView {
    // start with a set of three rows
    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 15,
        },
    ]);
    view! {
        // when we click, update each row,
        // doubling its value
        <button on:click=move |_| {
            set_data.update(|data| {
                for row in data {
                    row.value *= 2;
                }
            });
            // log the new value of the signal
            logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>
        // iterate over the rows and display each value
        <For
            each=data
            key=|state| state.key.clone()
            let:child
        >
            <p>{child.value}</p>
        </For>
    }
}