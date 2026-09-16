use crate::i18n::t;
use leptos::prelude::*;

#[component]
pub fn SharedVideoDialog(
    show: ReadSignal<bool>,
    on_close: Callback<()>,
    on_submit: Callback<String>,
) -> impl IntoView {
    let (url, set_url) = signal("".to_string());

    view! {
        <Show when=move || show.get()>
            <div class="modal-overlay" style="position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.5); display: flex; justify-content: center; align-items: center; z-index: 1000;">
                <div class="modal-content" style="width: 400px;">
                    <div class="modal-header">
                        <h3>{move || t("share_video")}</h3>
                        <button id="close-shared-video-btn" class="modal-close-btn" on:click=move |_| on_close.run(())>"×"</button>
                    </div>

                    <div class="form-group" style="margin-bottom: 15px;">
                        <label style="display: block; margin-bottom: 5px;">{move || t("youtube_url")}</label>
                        <input
                            type="text"
                            prop:value=url
                            on:input=move |ev| set_url.set(event_target_value(&ev))
                            placeholder="https://www.youtube.com/watch?v=..."
                            style="width: 100%;"
                        />
                    </div>

                    <div style="display: flex; justify-content: flex-end; gap: 10px;">
                        <button
                            class="btn btn-secondary"
                            on:click=move |_| on_close.run(())
                        >
                            {move || t("cancel")}
                        </button>
                        <button
                            id="submit-shared-video-btn"
                            class="btn btn-primary"
                            on:click=move |_| {
                                on_submit.run(url.get());
                                on_close.run(());
                                set_url.set("".to_string());
                            }
                        >
                            {move || t("share")}
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_video_dialog_compiles() {
        // Minimal test to ensure the component definition is valid Rust
        let _ = SharedVideoDialog;
    }
}
