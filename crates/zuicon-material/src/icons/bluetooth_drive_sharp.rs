// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BluetoothDriveSharp)]
pub fn bluetooth_drive_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BluetoothDriveSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M15,10H4.81l1.04-3H15V5H4.43L2,12v9h3v-2h12v2h3v-9h-3C15.9,12,15,11.1,15,10z M6.5,16C5.67,16,5,15.33,5,14.5 S5.67,13,6.5,13S8,13.67,8,14.5S7.33,16,6.5,16z M15.5,16c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5 S16.33,16,15.5,16z"/><path d="M22,3.85L19.15,1h-0.5v3.79l-2.3-2.29l-0.7,0.7L18.44,6l-2.79,2.79l0.7,0.71l2.3-2.3V11h0.5L22,8.14L19.85,6L22,3.85z M19.65,2.91l0.94,0.94l-0.94,0.94V2.91z M20.59,8.14l-0.94,0.94V7.2L20.59,8.14z"/>
        </SvgIcon>
    }
}
