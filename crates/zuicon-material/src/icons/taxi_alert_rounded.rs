// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TaxiAlertRounded)]
pub fn taxi_alert_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TaxiAlertRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,13c-1.91,0-3.63-0.76-4.89-2H4.81l1.04-3h5.44C11.1,7.37,11,6.7,11,6s0.1-1.37,0.29-2H9C8.45,4,8,4.45,8,5v1H5.5 C4.84,6,4.29,6.42,4.08,7.01L2,13v7.5C2,21.32,2.67,22,3.5,22S5,21.32,5,20.5V20h12v0.5c0,0.82,0.67,1.5,1.5,1.5 s1.5-0.68,1.5-1.5V13l-0.09-0.27C19.3,12.9,18.66,13,18,13z M6.5,17C5.67,17,5,16.33,5,15.5S5.67,14,6.5,14S8,14.67,8,15.5 S7.33,17,6.5,17z M15.5,17c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S16.33,17,15.5,17z"/><path d="M18,1c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,1,18,1z M18,9c-0.28,0-0.5-0.22-0.5-0.5S17.72,8,18,8 s0.5,0.22,0.5,0.5S18.28,9,18,9z M18.5,6.5C18.5,6.78,18.28,7,18,7s-0.5-0.22-0.5-0.5v-3C17.5,3.22,17.72,3,18,3 s0.5,0.22,0.5,0.5V6.5z"/>
        </SvgIcon>
    }
}
