// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HeadsetOffOutlined)]
pub fn headset_off_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HeadsetOffOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,4c3.87,0,7,3.13,7,7v1h-4v0.17L16.83,14H19v2.17l2,2V11c0-4.97-4.03-9-9-9C9.98,2,8.12,2.67,6.62,3.8l1.43,1.43 C9.17,4.45,10.53,4,12,4z"/><path d="M2.1,2.1L0.69,3.51l3.33,3.33C3.37,8.09,3,9.5,3,11v7c0,1.1,0.9,2,2,2h4v-8H5v-1c0-0.94,0.19-1.83,0.52-2.65L15,17.83V20 h2.17l1,1H12v2h7c0.34,0,0.65-0.09,0.93-0.24l0.55,0.55l1.41-1.41L2.1,2.1z M7,14v4H5v-4H7z"/>
        </SvgIcon>
    }
}
