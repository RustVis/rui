// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CallMissedOutgoingTwoTone)]
pub fn call_missed_outgoing_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CallMissedOutgoingTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0z" fill="none"/><path d="M19 10.41V15h2V7h-8v2h4.59L12 14.59 4.41 7 3 8.41l9 9z"/>
        </SvgIcon>
    }
}
