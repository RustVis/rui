// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SignalWifiOffRounded)]
pub fn signal_wifi_off_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SignalWifiOffRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M23.64 7c-.45-.34-4.93-4-11.64-4-1.32 0-2.55.14-3.69.38L18.43 13.5 23.64 7zM4.12 2.01c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41l1.35 1.35C1.91 5.76.59 6.82.36 7l10.08 12.56c.8 1 2.32 1 3.12 0l2.35-2.93 2.61 2.61c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L4.12 2.01z"/>
        </SvgIcon>
    }
}
