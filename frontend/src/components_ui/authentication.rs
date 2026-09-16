use leptos::prelude::*;

#[component]
pub fn LoginDialog(
    #[prop(into)] auth_error: Signal<Option<String>>,
    on_login: Callback<(String, Option<String>)>,
    on_cancel: Callback<()>,
) -> impl IntoView {
    let (username, set_username) = signal(String::new());
    let (password, set_password) = signal(String::new());

    let handle_submit = move |_| {
        let u = username.get();
        let p = password.get();
        if !u.is_empty() && !p.is_empty() {
            let pass = Some(p);
            on_login.run((u, pass));
        }
    };

    view! {
        <div class="login-dialog-overlay">
            <div class="modal-content login-dialog" style="width: 320px;">
                <div class="modal-header">
                    <h3>"Authentication Required"</h3>
                    <button class="modal-close-btn" on:click=move |_| on_cancel.run(())>"×"</button>
                </div>
                <div class="form-group" style="margin-bottom: 15px;">
                    <label class="form-label">"Username"</label>
                    <input
                        type="text"
                        class="login-username-input"
                        placeholder="user@domain.com"
                        on:input=move |ev| set_username.set(event_target_value(&ev))
                        prop:value=username
                        style="width: 100%;"
                    />
                </div>
                <div class="form-group" style="margin-bottom: 15px;">
                    <label class="form-label">"Password"</label>
                    <input
                        type="password"
                        class="login-password-input"
                        placeholder="Password"
                        on:input=move |ev| set_password.set(event_target_value(&ev))
                        prop:value=password
                        style="width: 100%;"
                    />
                </div>
                <Show when=move || auth_error.get().is_some()>
                    <div class="settings-error">
                        {move || auth_error.get().unwrap_or_default()}
                    </div>
                </Show>
                <div style="display: flex; justify-content: flex-end; gap: 10px;">
                    <button
                        class="btn btn-secondary login-cancel-btn"
                        on:click=move |_| on_cancel.run(())
                    >
                        "Cancel"
                    </button>
                    <button
                        class="btn btn-primary login-submit-btn"
                        on:click=handle_submit
                        disabled=move || username.get().is_empty() || password.get().is_empty()
                    >
                        "Login"
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
    fn test_login_dialog_compiles() {
        let owner = Owner::new();
        owner.set();
        let (auth_error, _set_auth_error) = signal::<Option<String>>(None);
        let on_login = Callback::new(|_: (String, Option<String>)| {});
        let on_cancel = Callback::new(|_: ()| {});

        let _view = LoginDialog(LoginDialogProps {
            auth_error: auth_error.into(),
            on_login,
            on_cancel,
        });
        let _ = true; // Verifies that instantiation succeeds within a reactive scope
    }
}
