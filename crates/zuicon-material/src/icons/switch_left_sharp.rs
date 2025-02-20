// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SwitchLeftSharp)]
pub fn switch_left_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SwitchLeftSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8.5,8.62v6.76L5.12,12L8.5,8.62 M10,5l-7,7l7,7V5L10,5z M14,5v14l7-7L14,5z"/>
        </SvgIcon>
    }
}
