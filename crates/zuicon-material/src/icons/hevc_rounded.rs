// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HevcRounded)]
pub fn hevc_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HevcRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6.25,9C5.84,9,5.5,9.34,5.5,9.75V11h-1V9.75C4.5,9.34,4.16,9,3.75,9S3,9.34,3,9.75v4.5C3,14.66,3.34,15,3.75,15 s0.75-0.34,0.75-0.75V12.5h1v1.75C5.5,14.66,5.84,15,6.25,15S7,14.66,7,14.25v-4.5C7,9.34,6.66,9,6.25,9z"/><path d="M10.75,10.5c0.41,0,0.75-0.34,0.75-0.75C11.5,9.34,11.16,9,10.75,9H9c-0.55,0-1,0.45-1,1v4c0,0.55,0.45,1,1,1h1.75 c0.41,0,0.75-0.34,0.75-0.75c0-0.41-0.34-0.75-0.75-0.75H9.5v-1h1.25c0.41,0,0.75-0.34,0.75-0.75c0-0.41-0.34-0.75-0.75-0.75H9.5 v-0.5H10.75z"/><path d="M15.63,9c-0.36,0-0.67,0.26-0.73,0.62l-0.65,3.88L13.6,9.62C13.54,9.26,13.23,9,12.87,9c-0.46,0-0.8,0.41-0.73,0.86 l0.65,3.91c0.12,0.71,0.73,1.23,1.46,1.23s1.34-0.52,1.46-1.23l0.65-3.91C16.43,9.41,16.08,9,15.63,9z"/><path d="M19.5,10.5c0,0.28,0.22,0.5,0.5,0.5h0.5c0.28,0,0.5-0.22,0.5-0.5V10c0-0.55-0.45-1-1-1h-2c-0.55,0-1,0.45-1,1v4 c0,0.55,0.45,1,1,1h2c0.55,0,1-0.45,1-1v-0.5c0-0.28-0.22-0.5-0.5-0.5H20c-0.28,0-0.5,0.22-0.5,0.5h-1v-3H19.5z"/>
        </SvgIcon>
    }
}
