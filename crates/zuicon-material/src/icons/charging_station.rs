// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ChargingStation)]
pub fn charging_station(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ChargingStation"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14.5,11l-3,6v-4h-2l3-6v4H14.5z M7,1h10c1.1,0,2,0.9,2,2v18c0,1.1-0.9,2-2,2H7c-1.1,0-2-0.9-2-2V3C5,1.9,5.9,1,7,1z M7,6 v12h10V6H7z"/>
        </SvgIcon>
    }
}
