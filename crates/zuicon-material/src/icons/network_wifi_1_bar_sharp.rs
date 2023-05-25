// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NetworkWifi1BarSharp)]
pub fn network_wifi_1_bar_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NetworkWifi1BarSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,4C7.31,4,3.07,5.9,0,8.98L12,21L24,8.98C20.93,5.9,16.69,4,12,4z M15.32,14.84C14.34,14.3,13.2,14,12,14 c-1.2,0-2.34,0.3-3.32,0.84L2.92,9.07C5.51,7.08,8.67,6,12,6s6.49,1.08,9.08,3.07L15.32,14.84z"/>
        </SvgIcon>
    }
}
