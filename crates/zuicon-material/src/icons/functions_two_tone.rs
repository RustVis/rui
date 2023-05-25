// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FunctionsTwoTone)]
pub fn functions_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FunctionsTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M18 17h-7l5-5-5-5h7V4H6v2l6.5 6L6 18v2h12z"/>
        </SvgIcon>
    }
}
