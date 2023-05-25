// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DirectionsCarFilledTwoTone)]
pub fn directions_car_filled_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DirectionsCarFilledTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,17h14v-5H5V17z M16.5,13c0.83,0,1.5,0.67,1.5,1.5S17.33,16,16.5,16 S15,15.33,15,14.5S15.67,13,16.5,13z M7.5,13C8.33,13,9,13.67,9,14.5S8.33,16,7.5,16S6,15.33,6,14.5S6.67,13,7.5,13z" enable-background="new" opacity=".3"/><path d="M18.92,6.01C18.72,5.42,18.16,5,17.5,5h-11C5.84,5,5.29,5.42,5.08,6.01L3,12v8c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-1 h12v1c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-8L18.92,6.01z M6.85,7h10.29l1.04,3H5.81L6.85,7z M19,17H5v-5h14V17z"/>
        </SvgIcon>
    }
}
