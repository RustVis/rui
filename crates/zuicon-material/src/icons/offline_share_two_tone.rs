// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OfflineShareTwoTone)]
pub fn offline_share_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("OfflineShareTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M6,5H4v16c0,1.1,0.9,2,2,2h10v-2H6V5z"/><path d="M18,1h-8C8.9,1,8,1.9,8,3v14c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2V3C20,1.9,19.1,1,18,1z M18,17h-8v-1h8V17z M18,14h-8V6h8 V14z M18,4h-8V3h8V4z"/><path d="M12.5,10.25h2V12L17,9.5L14.5,7v1.75H12c-0.55,0-1,0.45-1,1V12h1.5V10.25z"/>
        </SvgIcon>
    }
}
