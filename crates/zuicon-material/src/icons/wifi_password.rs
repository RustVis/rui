// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WifiPassword)]
pub fn wifi_password(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WifiPassword"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M23,19v-1c0-1.1-0.9-2-2-2s-2,0.9-2,2v1c-0.55,0-1,0.45-1,1v3c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-3 C24,19.45,23.55,19,23,19z M22,19h-2v-1c0-0.55,0.45-1,1-1s1,0.45,1,1V19z M24,8.98l-2.12,2.13C19.35,8.57,15.85,7,12,7 s-7.35,1.57-9.88,4.11L0,8.98C3.07,5.9,7.31,4,12,4S20.93,5.9,24,8.98z M12,10c3.03,0,5.78,1.23,7.76,3.22l-2.12,2.12 C16.2,13.9,14.2,13,12,13c-2.2,0-4.2,0.9-5.64,2.35l-2.12-2.12C6.22,11.23,8.97,10,12,10z M15.53,17.46L12,21l-3.53-3.54 C9.37,16.56,10.62,16,12,16S14.63,16.56,15.53,17.46z"/>
        </SvgIcon>
    }
}
