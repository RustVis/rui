// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ConnectWithoutContactRounded)]
pub fn connect_without_contact_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ConnectWithoutContactRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M7,4c0-1.11-0.89-2-2-2S3,2.89,3,4s0.89,2,2,2S7,5.11,7,4z M10.19,4.5L10.19,4.5c-0.41,0-0.76,0.25-0.92,0.63 C8.83,6.23,7.76,7,6.5,7h-3C2.67,7,2,7.67,2,8.5V11h6V8.74c1.43-0.45,2.58-1.53,3.12-2.91C11.38,5.19,10.88,4.5,10.19,4.5z M19,17 c1.11,0,2-0.89,2-2s-0.89-2-2-2s-2,0.89-2,2S17.89,17,19,17z M20.5,18h-3c-1.26,0-2.33-0.77-2.77-1.87 c-0.15-0.38-0.51-0.63-0.92-0.63h0c-0.69,0-1.19,0.69-0.94,1.33c0.55,1.38,1.69,2.46,3.12,2.91V22h6v-2.5C22,18.67,21.33,18,20.5,18 z M17.25,11.09c0,0,0-0.01,0.01,0c-1.06,0.27-1.9,1.11-2.17,2.17c0,0,0-0.01,0-0.01C14.98,13.68,14.58,14,14.11,14 c-0.55,0-1-0.45-1-1c0-0.05,0.02-0.14,0.02-0.14c0.43-1.85,1.89-3.31,3.75-3.73c0.04,0,0.08-0.01,0.12-0.01c0.55,0,1,0.45,1,1 C18,10.58,17.68,10.98,17.25,11.09z M18,6.06c0,0.51-0.37,0.92-0.86,0.99c0,0,0,0,0,0c-3.19,0.39-5.7,2.91-6.09,6.1c0,0,0,0,0,0 C10.98,13.63,10.56,14,10.06,14c-0.55,0-1-0.45-1-1c0-0.02,0-0.04,0-0.06c0-0.01,0-0.02,0-0.03c0.5-4.12,3.79-7.38,7.92-7.85 c0,0,0.01,0,0.01,0C17.55,5.06,18,5.51,18,6.06z"/>
        </SvgIcon>
    }
}
