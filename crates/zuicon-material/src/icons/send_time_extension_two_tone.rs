// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SendTimeExtensionTwoTone)]
pub fn send_time_extension_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SendTimeExtensionTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,6V4c0-0.28-0.22-0.5-0.5-0.5S11,3.72,11,4v2H5.01v2.13C7.17,8.94,8,11.01,8,12.5 c0,1.5-0.83,3.57-3,4.37V19h2.13c0.71-1.93,2.44-2.8,3.87-2.97V12V8.76l2.89,1.45L18,12.26V6H12z" opacity=".3"/><path d="M7.13,19H5v-2.13c2.17-0.8,3-2.87,3-4.37c0-1.49-0.83-3.56-2.99-4.37V6H11V4c0-0.28,0.22-0.5,0.5-0.5S12,3.72,12,4v2h6 v6.26l2,1V6c0-1.1-0.9-2-2-2h-4c0-1.38-1.12-2.5-2.5-2.5S9,2.62,9,4H5.01c-1.1,0-2,0.9-2,2v3.8C5.7,9.8,6,11.96,6,12.5 c0,0.54-0.29,2.7-3,2.7V19c0,1.1,0.9,2,2,2h3.8c0-2.16,1.37-2.78,2.2-2.94v-2.03C9.57,16.2,7.85,17.07,7.13,19z"/>
        </SvgIcon>
    }
}
