// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CameraswitchTwoTone)]
pub fn cameraswitch_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CameraswitchTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M13.17,8h-2.34l-1,1H8v6h8V9h-1.83L13.17,8z M12,14c-1.1,0-2-0.9-2-2c0-1.1,0.9-2,2-2s2,0.9,2,2 C14,13.1,13.1,14,12,14z" opacity=".3"/><path d="M16,7h-1l-1-1h-4L9,7H8C6.9,7,6,7.9,6,9v6c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2V9C18,7.9,17.1,7,16,7z M16,15H8V9h1.83l1-1 h2.34l1,1H16V15z"/><path d="M8.57,0.52L13.05,5V2.05C17.77,2.52,21.53,6.28,22,11c0,0,2,0,2,0C23.34,3.03,15.49-1.58,8.57,0.52z"/><path d="M10.95,21.96C6.23,21.49,2.47,17.73,2,13.01c0,0-2,0-2,0c0.66,7.97,8.51,12.58,15.43,10.48l-4.48-4.48V21.96z"/>
        </SvgIcon>
    }
}
