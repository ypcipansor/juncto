use leptos::*;

#[component]
pub fn CalendarList(
    #[prop(into)] events: Signal<Vec<String>>,
    on_refresh: Callback<()>,
    on_close: Callback<()>,
) -> impl IntoView {
    // Automatically refresh on mount
    create_effect(move |_| {
        // Trigger fetch asynchronously to avoid potential sync borrow issues during mount
        set_timeout(
            move || {
                on_refresh.call(());
            },
            std::time::Duration::from_millis(100),
        );
    });

    view! {
        <div class="calendar-list-overlay" style="position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;">
            <div class="calendar-list-dialog" style="width: 400px;">
                <div class="modal-header">
                    <h3>"Upcoming Meetings"</h3>
                    <button
                        class="modal-close-btn"
                        on:click=move |_| on_close.call(())
                    >
                        "×"
                    </button>
                </div>

                <div style="flex: 1; overflow-y: auto; margin-bottom: 15px;">
                    {move || if events.get().is_empty() {
                        view! {
                            <div class="calendar-empty">
                                "No upcoming events"
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <ul style="list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 10px;">
                                {
                                    let events_list = events.get().into_iter().enumerate().collect::<Vec<_>>();
                                    view! {
                                        <For
                                            each=move || events_list.clone()
                                            key=|(i, _)| *i
                                            children=move |(_i, evt)| {
                                                view! {
                                                    <li class="calendar-event">
                                                        {evt}
                                                    </li>
                                                }
                                            }
                                        />
                                    }
                                }
                            </ul>
                        }.into_view()
                    }}
                </div>

                <div style="display: flex; justify-content: flex-end;">
                    <button
                        class="btn btn-primary"
                        on:click=move |_| on_refresh.call(())
                    >
                        "Refresh"
                    </button>
                </div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_list_compiles() {
        let _ = create_runtime();
        let (events, _set_events) = create_signal::<Vec<String>>(Vec::new());
        let on_refresh = Callback::new(|_: ()| {});
        let on_close = Callback::new(|_: ()| {});

        let _view = CalendarList(CalendarListProps {
            events: events.into(),
            on_refresh,
            on_close,
        });
        let _ = true; // Verifies that instantiation succeeds within a reactive scope
    }
}
