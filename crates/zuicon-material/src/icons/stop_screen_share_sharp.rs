// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(StopScreenShareSharp)]
pub fn stop_screen_share_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("StopScreenShareSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0z" fill="none"/><path d="M21.79 18l2 2H24v-2zM13 9.13V7l4 3.74-1.28 1.19 5.18 5.18L22 16V4.02H7.8l5.13 5.13c.03-.01.05-.02.07-.02zM1.11 2.98l.89.9v12.14l2 1.99L0 18v2h18.13l2.71 2.71 1.41-1.41L2.52 1.57 1.11 2.98zm7.97 7.97l1.59 1.59C9.13 12.92 7.96 13.71 7 15c.31-1.48.94-2.93 2.08-4.05z"/>
        </SvgIcon>
    }
}
