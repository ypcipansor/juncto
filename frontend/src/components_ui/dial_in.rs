use leptos::*;

// Default placeholder values for the UI scaffold. Real dial-in details are
// not yet provisioned by the backend — when a telephony provider is
// integrated, the caller should pass `phone_number` and `meeting_id` props
// sourced from the room configuration (e.g. `RoomConfig`) rather than
// relying on these defaults.
const DEFAULT_DIAL_IN_PHONE: &str = "+1 555 012 3456";
const DEFAULT_DIAL_IN_MEETING_ID: &str = "123 456 789";

#[component]
pub fn DialInDialog(
    show: ReadSignal<bool>,
    on_close: Callback<()>,
    #[prop(into, optional)] phone_number: Option<Signal<Option<String>>>,
    #[prop(into, optional)] meeting_id: Option<Signal<Option<String>>>,
) -> impl IntoView {
    let phone_text = move || {
        phone_number
            .and_then(|s| s.get())
            .unwrap_or_else(|| DEFAULT_DIAL_IN_PHONE.to_string())
    };
    let meeting_text = move || {
        meeting_id
            .and_then(|s| s.get())
            .unwrap_or_else(|| DEFAULT_DIAL_IN_MEETING_ID.to_string())
    };
    view! {
        <Show when=move || show.get()>
            <div class="modal-overlay" style="position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.5); display: flex; justify-content: center; align-items: center; z-index: 1000;">
                <div class="modal-content" style="width: 400px; text-align: center;">
                    <div class="modal-header">
                        <h3>"Dial-in Information"</h3>
                        <button class="modal-close-btn" on:click=move |_| on_close.call(())>"×"</button>
                    </div>

                    <div class="dial-in-box">
                        <p class="dial-in-label">"To join by phone, dial one of these numbers:"</p>
                        <div class="dial-in-number">
                            {phone_text}
                        </div>
                        <p class="dial-in-label">"Meeting ID:"</p>
                        <div class="dial-in-id">
                            {meeting_text}
                        </div>
                    </div>

                    <p class="dial-in-note">
                        "Standard call rates apply. International numbers are available in the meeting invitation."
                    </p>

                    <button
                        id="dial-in-close-btn"
                        class="btn btn-secondary"
                        on:click=move |_| on_close.call(())
                        style="margin-top: 20px; width: 100%;"
                    >
                        "Close"
                    </button>
                </div>
            </div>
        </Show>
    }
}
