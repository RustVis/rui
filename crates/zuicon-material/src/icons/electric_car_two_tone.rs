// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ElectricCarTwoTone)]
pub fn electric_car_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ElectricCarTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,13h14V8H5V13z M16.5,9c0.83,0,1.5,0.67,1.5,1.5c0,0.83-0.67,1.5-1.5,1.5S15,11.33,15,10.5 C15,9.67,15.67,9,16.5,9z M7.5,9C8.33,9,9,9.67,9,10.5C9,11.33,8.33,12,7.5,12S6,11.33,6,10.5C6,9.67,6.67,9,7.5,9z" opacity=".3"/><path d="M18.92,2.01C18.72,1.42,18.16,1,17.5,1h-11C5.84,1,5.29,1.42,5.08,2.01L3,8v8c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-1 h12v1c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1V8L18.92,2.01z M6.85,3h10.29l1.08,3.11H5.77L6.85,3z M19,13H5V8h14V13z"/>
        </SvgIcon>
    }
}
