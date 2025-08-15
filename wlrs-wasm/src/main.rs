mod env;
mod status;
mod util;

use status::*;
use util::*;

use yew::{Html, function_component, html, use_state_eq};

#[allow(non_snake_case)]
#[function_component]
fn App() -> Html {
    let user = use_state_eq(String::new);
    let pass = use_state_eq(String::new);

    let label_opt = use_state_eq(|| StatusKind::Initial);

    let oninput_user = on_input(user.clone(), label_opt.clone());
    let oninput_pass = on_input(pass.clone(), label_opt.clone());
    let onsubmit = on_submit(user.clone(), pass.clone(), label_opt.clone());

    let onmouseover = on_mouse_oo("#6b6b6b");
    let onmouseout = on_mouse_oo("#5a5a5a");

    html! {
        <div style="
            display: flex; 
            width: 100vw; 
            height: 100vh; 
            justify-content: center; 
            align-items: center; 
            background: linear-gradient(#3c763d, #1b1b1b);
            font-family: 'Press Start 2P', cursive;
            font-size: 1.4rem; /* Scales all text */
        ">
            <div style="
                min-width: 44rem; /* doubled */
                background-color: #2b2b2b; 
                padding: 4rem 6rem; /* doubled */
                border-radius: 12px; /* slightly larger */
                border: 6px solid #5a5a5a; 
                box-shadow: 0 0 30px #000000aa;
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                gap: 3rem; /* doubled gap between title and form */
            ">
                <h2 style="
                    color: #71b039; 
                    text-shadow: 4px 4px #000; /* doubled shadow */
                    font-size: 3rem; /* doubled title size */
                    margin: 0;
                ">
                    { "Server Whitelister" }
                </h2>

                <form {onsubmit} style="
                    width: 100%; 
                    display: flex; 
                    flex-direction: column; 
                    align-items: center; 
                    justify-content: center;
                    gap: 2rem; /* doubled gap between inputs/buttons */
                ">
                    <input
                        oninput={oninput_user}
                        type="text"
                        placeholder="Username"
                        style="
                            width: 80%; /* slightly wider */
                            height: 6rem; /* doubled height */
                            background-color: #3b3b3b; 
                            color: white; 
                            border-radius: 8px; /* doubled */
                            border: 6px solid #736b5e; 
                            font-size: 1.2rem; /* bigger text */
                            text-align: center;
                            box-shadow: inset 0 0 10px #000; /* doubled shadow */
                        "
                    />
                    <input
                        oninput={oninput_pass}
                        type="password"
                        placeholder="Password"
                        style="
                            width: 80%; 
                            height: 6rem; 
                            background-color: #3b3b3b; 
                            color: white; 
                            border-radius: 8px; 
                            border: 6px solid #736b5e; 
                            font-size: 1.2rem; 
                            text-align: center;
                            box-shadow: inset 0 0 10px #000;
                        "
                    />
                    <input
                        type="submit"
                        value="SUBMIT"
                        style="
                            width: 60%; 
                            height: 5rem; /* doubled */
                            background-color: #5a5a5a; 
                            color: white; 
                            border-radius: 8px; 
                            border: 6px solid #736b5e; 
                            font-size: 1.2rem;
                            text-shadow: 2px 2px #000;
                            cursor: pointer;
                        "
                        onmouseover={onmouseover}
                        onmouseout={onmouseout}
                    />

                    // Fixed space for label so layout doesn't move
                    <div style="
                        height: 3rem; /* slightly taller for larger text */
                        display: flex;
                        align-items: center;
                        justify-content: center;
                    ">
                        if label_opt.is_new() {
                            { label_opt.as_html() }
                        }
                    </div>
                </form>
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
