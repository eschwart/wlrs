use web_sys::{
    BinaryType, HtmlInputElement, MessageEvent, WebSocket,
    js_sys::{ArrayBuffer, Uint8Array},
    wasm_bindgen::{JsCast, closure::Closure},
};
use yew::{InputEvent, MouseEvent, SubmitEvent, TargetCast, UseStateHandle};

use crate::{env::*, status::*};

pub fn on_mouse_oo(c: &'static str) -> impl Fn(MouseEvent) {
    |ms| {
        let target = ms
            .target_dyn_into::<HtmlInputElement>()
            .expect("event target should be an element");
        target
            .style()
            .set_property("background-color", c)
            .expect("Failed to change 'background-color' property.");
    }
}

pub fn on_input(
    text: UseStateHandle<String>,
    label: UseStateHandle<StatusKind>,
) -> impl Fn(InputEvent) {
    move |event: InputEvent| {
        if let Some(input) = event.target_dyn_into::<HtmlInputElement>() {
            text.set(input.value())
        } else {
            log::error!("{}", StatusKind::InvalidInput.as_str());
            label.set(StatusKind::InvalidInput)
        }
    }
}

pub fn on_submit(
    user: UseStateHandle<String>,
    pass: UseStateHandle<String>,
    label: UseStateHandle<StatusKind>,
) -> impl Fn(SubmitEvent) {
    move |event: SubmitEvent| {
        // prevent page reload
        event.prevent_default();

        // current text input
        let user = user.trim();
        let pass = pass.trim();

        // discontinue if the input is empty
        if user.is_empty() || pass.is_empty() {
            return label.set(StatusKind::InvalidInput);
        }

        // only communicate with server if input exists
        let ws = match WebSocket::new(WEBSOCKET_ADDR) {
            Ok(s) => s,
            Err(_) => {
                log::error!("{}", StatusKind::Connection.as_str());
                return label.set(StatusKind::Connection);
            }
        };
        ws.set_binary_type(BinaryType::Arraybuffer);

        {
            let label = label.clone();
            let ws_onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
                let res = e
                    .data()
                    .dyn_into::<ArrayBuffer>()
                    .map(|buf| Uint8Array::new(&buf).to_vec())
                    .unwrap_or(vec![4]);

                let kind = StatusKind::from_u8(res.first().unwrap_or(&4));
                label.set(kind)
            }) as Box<dyn Fn(_)>);

            ws.set_onmessage(Some(ws_onmessage.as_ref().unchecked_ref()));
            ws_onmessage.forget();
        }

        {
            let user = user.to_owned();
            let pass = pass.to_owned();
            let label = label.clone();
            let ws_clone = ws.clone();
            let ws_onopen = Closure::wrap(Box::new(move || {
                label.set(StatusKind::Connecting);
                let data = [
                    [user.len() as u8].as_slice(),
                    user.as_bytes(),
                    pass.as_bytes(),
                ]
                .concat();
                _ = ws_clone.send_with_u8_array(data.as_slice());
            }) as Box<dyn Fn()>);
            ws.set_onopen(Some(ws_onopen.as_ref().unchecked_ref()));
            ws_onopen.forget();
        }

        {
            let label = label.clone();
            let ws_clone = ws.clone();
            let ws_onerror = Closure::wrap(Box::new(move || {
                label.set(StatusKind::Connection);
                _ = ws_clone.close()
            }) as Box<dyn Fn()>);
            ws.set_onerror(Some(ws_onerror.as_ref().unchecked_ref()));
            ws_onerror.forget();
        }
    }
}
