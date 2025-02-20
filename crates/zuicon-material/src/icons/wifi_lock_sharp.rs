// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WifiLockSharp)]
pub fn wifi_lock_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WifiLockSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M21.98,11L24,8.98C20.93,5.9,16.69,4,12,4C7.31,4,3.07,5.9,0,8.98l6.35,6.36L12,21l3.05-3.05V15 c0-0.45,0.09-0.88,0.23-1.29c0.54-1.57,2.01-2.71,3.77-2.71H21.98z"/><path d="M22,15.11c0-1-0.68-1.92-1.66-2.08c-0.12-0.02-0.24-0.02-0.36-0.02l0,0c0,0-0.01,0-0.01,0C18.88,13.03,18,13.91,18,15v1 c-0.55,0-1,0-1,0v5h6v-5c0,0-0.45,0-1,0V15.11z M21,16h-2v-1c0-0.55,0.45-1,1-1s1,0.45,1,1V16z"/>
        </SvgIcon>
    }
}
