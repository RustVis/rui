// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SendTimeExtensionSharp)]
pub fn send_time_extension_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SendTimeExtensionSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,4h-6c0-1.38-1.12-2.5-2.5-2.5S9,2.62,9,4H3.01v5.8C5.7,9.8,6,11.96,6,12.5c0,0.54-0.29,2.7-3,2.7V21h5.8 c0-2.16,1.37-2.78,2.2-2.94v-9.3l9,4.5V4z"/>
        </SvgIcon>
    }
}
