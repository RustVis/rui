// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HttpSharp)]
pub fn http_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HttpSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M24 24H0V0h24v24z" fill="none" opacity=".87"/><path d="M4.5 11h-2V9H1v6h1.5v-2.5h2V15H6V9H4.5v2zm2.5-.5h1.5V15H10v-4.5h1.5V9H7v1.5zm5.5 0H14V15h1.5v-4.5H17V9h-4.5v1.5zM23 9h-5v6h1.5v-2H23V9zm-1.5 2.5h-2v-1h2v1z"/>
        </SvgIcon>
    }
}
