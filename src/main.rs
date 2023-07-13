use leptos::*;
use rand::Rng;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (rendered, set_rendered) = create_signal(cx, false);
    let (updated, set_updated) = create_signal(cx, false);
    let (rows_value, set_rows_value) = create_signal(cx, 10);

    let initial_rows = (0..rows_value())
        .map(|id| (id, create_signal(cx, id + 1)))
        .collect::<Vec<_>>();

    let (rows, set_rows) = create_signal(cx, initial_rows);

    let change_rows = move || {
        set_rows.update(move |rows| {
            *rows = (0..rows_value())
                .map(|id| (id, create_signal(cx, id + 1)))
                .collect::<Vec<_>>()
        });
    };

    let update_rows = move |_| {
        set_updated(!updated());
    };

    view! { cx,
        <h1>"Welcome to Connect, but much better"</h1>
        <button
            on:click=move |_| {
                set_rendered(!rendered());
            }
        >
            "render rows"
        </button>
        <input type="number"
            on:input=move |ev| {
                set_rows_value(event_target_value(&ev).parse().unwrap());
                change_rows();
            }
            prop:value=rows_value
            value=rows_value
            min=1
        />
        <button
            on:click=update_rows
        >
            "update every 10th row"
        </button>
        <Show
            when=move || rendered()
            fallback=|_cx| view! {cx, }
        >
            <For
                each=rows
                key=|row| row.0
                view=move |cx, (id, (_row, _set_row))| {
                    view!{cx,
                    <div
                      class=("purple", move || _row() % 10 == 0 && updated())
                      style="display: flex; align-items: end"
                    >
                      <div>"Some text here: " {_row}</div>
                      <button
                        on:click=move |_| {
                            set_rows.update(|rows| {
                                set_rows_value.update(|val| *val-= 1);
                                rows.retain(|(row_id, _)| row_id != &id)
                            });
                        }
                        >"delete row"
                        </button>
                    </div>}
                }
            />
        </Show>
    }
}
