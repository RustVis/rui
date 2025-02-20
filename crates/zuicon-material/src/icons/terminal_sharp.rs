// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TerminalSharp)]
pub fn terminal_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TerminalSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,4v16h20V4H2z M20,18H4V8h16V18z M18,17h-6v-2h6V17z M7.5,17l-1.41-1.41L8.67,13l-2.59-2.59L7.5,9l4,4L7.5,17z"/>
        </SvgIcon>
    }
}
