// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewComfyAltTwoTone)]
pub fn view_comfy_alt_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewComfyAltTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,18h16V6H4V18z M13,7h4v4h-4V7z M13,13h4v4h-4V13z M7,7h4v4H7V7z M7,13h4v4H7V13z" opacity=".3"/><path d="M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6C22,4.9,21.1,4,20,4z M20,18H4V6h16V18z"/>
        </SvgIcon>
    }
}
