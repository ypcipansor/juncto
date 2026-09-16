use leptos::prelude::*;
use send_wrapper::SendWrapper;

/// Registers a cleanup function that may capture browser-only (`!Send`) values.
///
/// Leptos runs cleanup functions on the thread that owns the reactive graph, so
/// captured browser handles are only ever touched there. [`SendWrapper`] keeps
/// that guarantee explicit while satisfying Leptos' `Send + Sync` bound.
pub fn on_cleanup_local(fun: impl FnOnce() + 'static) {
    let owner = SendWrapper::new(Owner::current().expect("no current reactive Owner found"));
    let fun = SendWrapper::new(fun);
    on_cleanup(move || owner.with(fun.take()));
}
