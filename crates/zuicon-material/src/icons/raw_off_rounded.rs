// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RawOffRounded)]
pub fn raw_off_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RawOffRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.55,9c-0.33,0-0.63,0.23-0.71,0.55L19.24,12l-0.56-2.26C18.58,9.3,18.19,9,17.74,9S16.9,9.3,16.8,9.74L16.24,12 l-0.6-2.45C15.56,9.23,15.27,9,14.93,9c-0.47,0-0.82,0.44-0.71,0.9l0.5,1.99l2.42,2.42c0-0.01,0.01-0.02,0.01-0.03l0.58-2.32 l0.58,2.32C18.43,14.7,18.81,15,19.24,15s0.81-0.3,0.92-0.72l1.09-4.38C21.37,9.44,21.02,9,20.55,9z"/><path d="M3.51,3.51c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41L6.17,9H4c-0.55,0-1,0.45-1,1v4.31 C3,14.69,3.31,15,3.69,15h0.11c0.38,0,0.69-0.31,0.69-0.69V13h1.1l0.72,1.59C6.43,14.84,6.68,15,6.95,15 c0.5,0,0.83-0.51,0.64-0.97L7.1,12.9C7.6,12.6,8,12.1,8,11.5v-0.67l1.43,1.43L8.98,14.1C8.86,14.56,9.21,15,9.68,15h0 c0.33,0,0.62-0.23,0.7-0.55l0.24-0.95h0.04l8.4,8.4c0.39,0.39,1.02,0.39,1.41,0c0.39-0.39,0.39-1.02,0-1.41L3.51,3.51z M6.5,11.5 h-2v-1h2V11.5z"/>
        </SvgIcon>
    }
}
