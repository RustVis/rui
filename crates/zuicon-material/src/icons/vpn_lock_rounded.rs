// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VpnLockRounded)]
pub fn vpn_lock_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VpnLockRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19.92,11c0.44,3.63-1.52,5.85-2.02,6.39C17.64,16.58,16.9,16,16,16h-1v-3c0-0.55-0.45-1-1-1H8v-2h2c0.55,0,1-0.45,1-1V7h2 c1.1,0,2-0.9,2-2V2.46C14.05,2.16,13.05,2,12,2C6.48,2,2,6.48,2,12s4.48,10,10,10c5.73,0,10.51-4.86,9.95-11H19.92z M11,19.93 C7.05,19.44,4,16.08,4,12c0-0.62,0.08-1.21,0.21-1.79L9,15v1c0,1.1,0.9,2,2,2V19.93z M22,4V3.11c0-1-0.68-1.92-1.66-2.08 C19.08,0.82,18,1.79,18,3v1c-0.55,0-1,0.45-1,1v3c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1V5C23,4.45,22.55,4,22,4z M21,4h-2V3 c0-0.55,0.45-1,1-1s1,0.45,1,1V4z"/>
        </SvgIcon>
    }
}
