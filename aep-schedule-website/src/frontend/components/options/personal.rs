use crate::frontend::components::common::schedule::Schedule;
use leptos::*;
use web_sys::wasm_bindgen::JsCast;
use web_sys::Element;

#[component]
pub fn PersonalTimeSelector<F>(week: [RwSignal<u64>; 5], submit: F) -> impl IntoView
where
    F: Fn() + Copy + 'static,
{
    let (initial, set_initial) = create_signal(None);
    let (destination, set_destination) = create_signal((0, 0));
    let (is_positive, set_positive) = create_signal(true);
    let selection = move || {
        let Some((initial_x, initial_y)) = initial.get() else {
            return String::from("display: none;");
        };
        let (destination_x, destination_y) = destination.get();
        let [initial_x, destination_x] = std::cmp::minmax(initial_x, destination_x);
        let [initial_y, destination_y] = std::cmp::minmax(initial_y, destination_y);
        let height = destination_y - initial_y + 2;
        let width = destination_x - initial_x + 1;
        format!(
            "grid-column:{} / span {};grid-row:{} / span {};",
            initial_x + 3,
            width,
            initial_y + 5,
            height
        )
    };

    let handle = window_event_listener(ev::pointerup, move |_| {
        let Some((initial_x, initial_y)) = initial.get() else {
            return;
        };
        let (i, j) = destination.get();
        let [initial_x, destination_x] = std::cmp::minmax(initial_x, i);
        let [initial_y, destination_y] = std::cmp::minmax(initial_y, j);
        let is_positive = is_positive.get();
        for day in initial_x..=destination_x {
            week[day].update(|v| {
                if is_positive {
                    *v |= 2u64.pow(destination_y as u32 + 2) - 2u64.pow(initial_y as u32);
                } else {
                    *v &= !(2u64.pow(destination_y as u32 + 2) - 2u64.pow(initial_y as u32));
                }
            });
        }
        submit();
        set_initial.set(None);
    });
    on_cleanup(move || handle.remove());

    let selection_class = move || {
        if is_positive.get() {
            "selection selected-hour"
        } else {
            "selection unselected-hour"
        }
    };
    view! {
        <Schedule col_height="0.35em">
            {(0..5).into_iter().map(|i| {
                (0..26).into_iter().map(|j| {
                    let j = 2 * j;
                    let style = format!(
                        "grid-column:{};grid-row:{} / span {};",
                        i + 3,
                        j + 5,
                        2
                    );
                    let class = move || {
                        let day = week[i].get();
                        let hour = day & (1 << j);
                        if hour != 0 {
                            "touch-none selected-hour"
                        } else {
                            "touch-none"
                        }
                    };
                    view! {
                        <div style=style class=class
                            on:pointerdown=move |e| {
                                set_initial.set(Some((i, j)));
                                set_positive.set((week[i].get() & (1 << j)) == 0);
                                let _ = e.target().unwrap().dyn_ref::<Element>().unwrap().release_pointer_capture(e.pointer_id());
                            }
                            on:pointerover=move |_| {
                                set_destination.set((i, j));
                            }>
                        </div>
                    }
                }).collect_view()
            }).collect_view()}
            <div style=selection class=selection_class></div>
        </Schedule>
    }
}
