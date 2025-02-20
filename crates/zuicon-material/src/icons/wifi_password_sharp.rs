// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WifiPasswordSharp)]
pub fn wifi_password_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WifiPasswordSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M24,8.98l-2.12,2.13C19.35,8.57,15.85,7,12,7s-7.35,1.57-9.88,4.11L0,8.98C3.07,5.9,7.31,4,12,4S20.93,5.9,24,8.98z M4.24,13.22l2.12,2.12C7.8,13.9,9.8,13,12,13c2.2,0,4.2,0.9,5.64,2.35l2.12-2.12C17.78,11.23,15.03,10,12,10 C8.97,10,6.22,11.23,4.24,13.22z M24,19v5h-6v-5h1v-1c0-1.1,0.9-2,2-2s2,0.9,2,2v1H24z M22,18c0-0.55-0.45-1-1-1s-1,0.45-1,1v1h2 V18z M12,16c-1.38,0-2.63,0.56-3.53,1.46L12,21l3.53-3.54C14.63,16.56,13.38,16,12,16z"/>
        </SvgIcon>
    }
}
