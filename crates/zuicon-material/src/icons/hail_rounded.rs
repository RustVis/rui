// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HailRounded)]
pub fn hail_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HailRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,6c-1.1,0-2-0.9-2-2s0.9-2,2-2s2,0.9,2,2S13.1,6,12,6z M17.95,2L17.95,2c0.59,0,1.06,0.51,1,1.09 C18.93,3.24,18.74,7.15,15,8.4V21c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1v-5h-2v5c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1V10.1 c-0.3,0.1-0.5,0.2-0.6,0.3c-0.46,0.36-1.17,0.87-1.36,2.67C6.99,13.59,6.57,14,6.04,14h0c-0.58,0-1.05-0.49-1-1.07 c0.13-1.6,0.62-2.98,2.07-4.22C8.21,7.81,10,7,12,7s2.68-0.46,3.48-1.06c0.43-0.34,1.28-0.99,1.48-3.02C17.01,2.4,17.43,2,17.95,2z M5,16h1c0.55,0,1,0.45,1,1v4c0,0.55-0.45,1-1,1H5c-0.55,0-1-0.45-1-1v-4C4,16.45,4.45,16,5,16z"/>
        </SvgIcon>
    }
}
