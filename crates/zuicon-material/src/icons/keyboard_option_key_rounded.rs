// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(KeyboardOptionKeyRounded)]
pub fn keyboard_option_key_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("KeyboardOptionKeyRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15,6L15,6c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-4C15.45,5,15,5.45,15,6z"/><path d="M9.58,6C9.22,5.38,8.56,5,7.85,5H4C3.45,5,3,5.45,3,6v0c0,0.55,0.45,1,1,1h3.85l6.35,11c0.36,0.62,1.02,1,1.73,1H20 c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-4.07L9.58,6z"/>
        </SvgIcon>
    }
}
