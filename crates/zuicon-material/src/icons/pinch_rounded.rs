// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PinchRounded)]
pub fn pinch_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PinchRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8.2,17.43c0-0.65,0.6-1.13,1.24-0.99L13,17.24V6.5C13,5.67,13.67,5,14.5,5S16,5.67,16,6.5v6h0.91 c0.31,0,0.62,0.07,0.89,0.21l4.09,2.04c0.77,0.38,1.21,1.22,1.09,2.07l-0.63,4.46C22.21,22.27,21.36,23,20.37,23h-6.16 c-0.53,0-1.29-0.21-1.66-0.59l-4.07-4.29C8.3,17.94,8.2,17.69,8.2,17.43z M9.5,5.25C9.5,5.66,9.84,6,10.25,6S11,5.66,11,5.25V2 c0-0.55-0.45-1-1-1H6.75C6.34,1,6,1.34,6,1.75S6.34,2.5,6.75,2.5h1.69L2.5,8.44V6.75C2.5,6.34,2.16,6,1.75,6S1,6.34,1,6.75V10 c0,0.55,0.45,1,1,1h3.25C5.66,11,6,10.66,6,10.25S5.66,9.5,5.25,9.5H3.56L9.5,3.56V5.25z"/>
        </SvgIcon>
    }
}
