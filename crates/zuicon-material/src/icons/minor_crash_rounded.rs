// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MinorCrashRounded)]
pub fn minor_crash_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MinorCrashRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19.5,24c0.82,0,1.5-0.67,1.5-1.5v-7.16c0-0.22-0.04-0.45-0.11-0.66l-1.97-5.67C18.72,8.42,18.16,8,17.5,8h-11 C5.84,8,5.29,8.42,5.08,9.01l-1.97,5.67C3.04,14.89,3,15.11,3,15.34v7.16C3,23.33,3.68,24,4.5,24S6,23.33,6,22.5V22h12v0.5 C18,23.33,18.67,24,19.5,24z M6.85,10h10.29l1.04,3H5.81L6.85,10z M6,17.5C6,16.67,6.67,16,7.5,16S9,16.67,9,17.5S8.33,19,7.5,19 S6,18.33,6,17.5z M15,17.5c0-0.83,0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S17.33,19,16.5,19S15,18.33,15,17.5z M8.71,5.71 c-0.39,0.39-1.02,0.39-1.41,0L5.71,4.12c-0.39-0.39-0.39-1.02,0-1.41s1.02-0.39,1.41,0l1.59,1.59C9.1,4.68,9.1,5.32,8.71,5.71z M18.29,2.71c0.39,0.39,0.39,1.02,0,1.41l-1.59,1.59c-0.39,0.39-1.02,0.39-1.41,0s-0.39-1.02,0-1.41l1.59-1.59 C17.27,2.32,17.9,2.32,18.29,2.71z M12,5c-0.55,0-1-0.45-1-1V1c0-0.55,0.45-1,1-1s1,0.45,1,1v3C13,4.55,12.55,5,12,5z"/>
        </SvgIcon>
    }
}
