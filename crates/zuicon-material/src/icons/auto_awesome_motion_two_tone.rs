// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AutoAwesomeMotionTwoTone)]
pub fn auto_awesome_motion_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AutoAwesomeMotionTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14,2H4C2.9,2,2,2.9,2,4v10h2V4h10V2z"/><path d="M20,10h-8c-1.1,0-2,0.9-2,2v8c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2v-8C22,10.9,21.1,10,20,10z M20,20h-8v-8h8V20z"/><path d="M18,6H8C6.9,6,6,6.9,6,8v10h2V8h10V6z"/>
        </SvgIcon>
    }
}
