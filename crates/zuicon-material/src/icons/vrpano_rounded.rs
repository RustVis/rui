// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VrpanoRounded)]
pub fn vrpano_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VrpanoRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.69,4.05C18.66,4.73,15.86,5.5,12,5.5c-3.89,0-6.95-0.84-8.69-1.43C2.67,3.85,2,4.33,2,5.02V19 c0,0.68,0.66,1.17,1.31,0.95C5.36,19.26,8.1,18.5,12,18.5c3.87,0,6.66,0.76,8.69,1.45C21.34,20.16,22,19.68,22,19V5 C22,4.32,21.34,3.84,20.69,4.05z M17.28,15.26C15.62,15.1,13.84,15,12,15c-1.87,0-3.63,0.1-5.28,0.27 C6.27,15.31,6,14.79,6.29,14.45l2.5-3c0.2-0.24,0.57-0.24,0.77,0l1.62,1.94l2.44-2.93c0.2-0.24,0.57-0.24,0.77,0l3.32,3.99 C17.99,14.79,17.72,15.31,17.28,15.26z"/>
        </SvgIcon>
    }
}
