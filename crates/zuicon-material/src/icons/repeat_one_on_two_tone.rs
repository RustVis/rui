// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RepeatOneOnTwoTone)]
pub fn repeat_one_on_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RepeatOneOnTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,1H3C1.9,1,1,1.9,1,3v18c0,1.1,0.9,2,2,2h18c1.1,0,2-0.9,2-2V3C23,1.9,22.1,1,21,1z M19,19H7v3l-4-4l4-4v3h10v-4h2V19z M10,10.5V9h3v6h-1.5v-4.5H10z M17,10V7H7v4H5V5h12V2l4,4L17,10z"/>
        </SvgIcon>
    }
}
