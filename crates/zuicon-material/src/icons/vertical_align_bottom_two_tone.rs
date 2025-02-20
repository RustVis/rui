// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VerticalAlignBottomTwoTone)]
pub fn vertical_align_bottom_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VerticalAlignBottomTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M11 3v10H8l4 4 4-4h-3V3zM4 19h16v2H4z"/>
        </SvgIcon>
    }
}
