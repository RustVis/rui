// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OpacityTwoTone)]
pub fn opacity_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("OpacityTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M24 0H0v24h24V0zm0 0H0v24h24V0zM0 24h24V0H0v24z" fill="none"/><path d="M16.24 9.65L12 5.27 7.76 9.6C6.62 10.73 6.01 12 6 14h12c-.01-2-.62-3.23-1.76-4.35z" opacity=".3"/><path d="M17.66 8L12 2.35 6.34 8C4.78 9.56 4 11.64 4 13.64s.78 4.11 2.34 5.67 3.61 2.35 5.66 2.35 4.1-.79 5.66-2.35S20 15.64 20 13.64 19.22 9.56 17.66 8zM6 14c.01-2 .62-3.27 1.76-4.4L12 5.27l4.24 4.38C17.38 10.77 17.99 12 18 14H6z"/>
        </SvgIcon>
    }
}
