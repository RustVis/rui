// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FoundationRounded)]
pub fn foundation_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FoundationRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,12h1.7c0.46,0,0.68-0.57,0.33-0.87L12.67,3.6c-0.38-0.34-0.96-0.34-1.34,0l-8.36,7.53C2.63,11.43,2.84,12,3.3,12H5v3H4 c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h1v2c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-2h4v2c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1 v-2h4v2c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-2h1c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-1V12z M7,15v-4.81l4-3.6V15H7z M13,15 V6.59l4,3.6V15H13z"/>
        </SvgIcon>
    }
}
