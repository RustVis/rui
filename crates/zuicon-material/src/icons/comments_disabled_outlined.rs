// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CommentsDisabledOutlined)]
pub fn comments_disabled_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CommentsDisabledOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18.83,16H20V4H6.83l-2-2H20c1.1,0,2,0.9,2,2l0,15.17L18.83,16z M18,6H8.83l2,2H18V6z M18,9h-6.17l2,2H18V9z M18,14v-2 h-3.17l2,2H18z M21.9,21.9l-1.41,1.41L15.17,18H4c-1.1,0-2-0.9-2-2V4.83L0.69,3.51L2.1,2.1L21.9,21.9z M13.17,16l-2-2H6v-2h3.17 l-1-1H6V9h0.17L4,6.83V16H13.17z"/>
        </SvgIcon>
    }
}
