// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ShuffleOnOutlined)]
pub fn shuffle_on_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ShuffleOnOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,1H3C1.9,1,1,1.9,1,3v18c0,1.1,0.9,2,2,2h18c1.1,0,2-0.9,2-2V3C23,1.9,22.1,1,21,1z M5.41,4l5.18,5.17l-1.41,1.42 L4,5.42L5.41,4z M20,20h-6v-2h2.61l-3.2-3.2l1.42-1.42l3.13,3.13L18,16.55V14h2V20z M20,10h-2V7.42L5.41,20L4,18.59L16.58,6H14V4h6 V10z"/>
        </SvgIcon>
    }
}
