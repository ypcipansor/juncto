use leptos::prelude::*;

#[component]
pub fn LobbyScreen(announcement: ReadSignal<Option<String>>) -> impl IntoView {
    view! {
        <div class="lobby-container">
            <div class="lobby-card">
                <h2>"Waiting for host..."</h2>
                <p>"You have asked to join the meeting. Please wait for the host to let you in."</p>

                <Show when=move || announcement.get().is_some()>
                    <div class="lobby-announcement">
                        <strong>"Message from Host:"</strong>
                        <span>{move || announcement.get()}</span>
                    </div>
                </Show>

                <div class="lobby-spinner">"⏳"</div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lobby_screen_compiles() {
        let owner = Owner::new();
        owner.set();
        let (announcement, _) = signal(None::<String>);
        let _view = view! { <LobbyScreen announcement=announcement /> };
        let _ = true;
    }
}
