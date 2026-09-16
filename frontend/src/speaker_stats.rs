use leptos::prelude::*;
use shared::Participant;

#[component]
pub fn SpeakerStatsDialog(
    show: ReadSignal<bool>,
    participants: ReadSignal<Vec<Participant>>,
    on_close: Callback<()>,
) -> impl IntoView {
    let sorted_participants = move || {
        let mut p = participants.get();
        p.sort_by_key(|b| std::cmp::Reverse(b.speaking_time));
        p
    };

    let format_time = |ms: u64| {
        let seconds = ms / 1000;
        let m = seconds / 60;
        let s = seconds % 60;
        format!("{:02}:{:02}", m, s)
    };

    view! {
        <Show when=move || show.get()>
            <div class="modal-overlay" style="position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.5); display: flex; justify-content: center; align-items: center; z-index: 1000;">
                <div class="modal-content" style="width: 400px;">
                    <div class="modal-header">
                        <h3>"Speaker Stats"</h3>
                        <button id="close-speaker-stats-btn" class="modal-close-btn" on:click=move |_| on_close.run(())>"×"</button>
                    </div>
                    <table style="width: 100%; border-collapse: collapse;">
                        <thead>
                            <tr>
                                <th style="text-align: left;">"Name"</th>
                                <th style="text-align: right;">"Time"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <For
                                each=sorted_participants
                                key=|p| p.id.clone()
                                children=move |p| {
                                    view! {
                                        <tr>
                                            <td>{p.name}</td>
                                            <td style="text-align: right;">{format_time(p.speaking_time)}</td>
                                        </tr>
                                    }
                                }
                            />
                        </tbody>
                    </table>
                </div>
            </div>
        </Show>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_format_time_logic() {
        let format_time = |ms: u64| {
            let seconds = ms / 1000;
            let m = seconds / 60;
            let s = seconds % 60;
            format!("{:02}:{:02}", m, s)
        };

        assert_eq!(format_time(0), "00:00");
        assert_eq!(format_time(1000), "00:01");
        assert_eq!(format_time(61000), "01:01");
        assert_eq!(format_time(3600000), "60:00");
    }
}
