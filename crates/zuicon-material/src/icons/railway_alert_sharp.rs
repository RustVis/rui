// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RailwayAlertSharp)]
pub fn railway_alert_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RailwayAlertSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,11V8h7.29c-0.77-2.6,0.21-4.61,0.37-4.97C2.97,2.67,2,5.02,2,7v9.5C2,18.43,3.57,20,5.5,20L4,21v1h12v-1l-1.5-1 c1.93,0,3.5-1.57,3.5-3.5V13c-1.91,0-3.63-0.76-4.89-2H4z M10,17c-0.83,0-1.5-0.67-1.5-1.5c0-0.83,0.67-1.5,1.5-1.5 s1.5,0.67,1.5,1.5C11.5,16.33,10.83,17,10,17z"/><path d="M18,1c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,1,18,1z M18.5,9h-1V8h1V9z M18.5,7h-1V3h1V7z"/>
        </SvgIcon>
    }
}
