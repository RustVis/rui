// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AttachmentOutlined)]
pub fn attachment_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AttachmentOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M18.5 16H7c-2.21 0-4-1.79-4-4s1.79-4 4-4h12.5c1.38 0 2.5 1.12 2.5 2.5S20.88 13 19.5 13H9c-.55 0-1-.45-1-1s.45-1 1-1h9.5V9.5H9c-1.38 0-2.5 1.12-2.5 2.5s1.12 2.5 2.5 2.5h10.5c2.21 0 4-1.79 4-4s-1.79-4-4-4H7c-3.04 0-5.5 2.46-5.5 5.5s2.46 5.5 5.5 5.5h11.5V16z"/>
        </SvgIcon>
    }
}
