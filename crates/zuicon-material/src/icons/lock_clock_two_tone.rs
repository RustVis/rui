// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LockClockTwoTone)]
pub fn lock_clock_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LockClockTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M11.29,20H6V10h12v1c0.7,0,1.37,0.1,2,0.29V10c0-1.1-0.9-2-2-2h-1V6c0-2.76-2.24-5-5-5S7,3.24,7,6v2H6c-1.1,0-2,0.9-2,2 v10c0,1.1,0.9,2,2,2h6.26C11.84,21.4,11.51,20.72,11.29,20z M9,6c0-1.66,1.34-3,3-3s3,1.34,3,3v2H9V6z"/><path d="M11,18c0-3.87,3.13-7,7-7v-1H6v10h5.29C11.1,19.37,11,18.7,11,18z" enable-background="new" opacity=".3"/><path d="M18,13c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,13,18,13z M19.65,20.35l-2.15-2.15V15h1v2.79l1.85,1.85 L19.65,20.35z"/>
        </SvgIcon>
    }
}
