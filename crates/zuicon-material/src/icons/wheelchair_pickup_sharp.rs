// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WheelchairPickupSharp)]
pub fn wheelchair_pickup_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WheelchairPickupSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4.5,4c0-1.11,0.89-2,2-2s2,0.89,2,2s-0.89,2-2,2S4.5,5.11,4.5,4z M10,10.95V7H3v8h2v7h3.5v-0.11c-1.24-1.26-2-2.99-2-4.89 C6.5,14.42,7.91,12.16,10,10.95z M16.5,17c0,1.65-1.35,3-3,3s-3-1.35-3-3c0-1.11,0.61-2.06,1.5-2.58v-2.16 C9.98,12.9,8.5,14.77,8.5,17c0,2.76,2.24,5,5,5s5-2.24,5-5H16.5z M19.54,14H15V8h-2v8h5.46l2.47,3.71l1.66-1.11L19.54,14z"/>
        </SvgIcon>
    }
}
