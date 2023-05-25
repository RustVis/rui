// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SolarPowerRounded)]
pub fn solar_power_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SolarPowerRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3.33,16H11v-3H5.6c-0.94,0-1.75,0.65-1.95,1.57L3.33,16z"/><path d="M13,16h7.67l-0.32-1.43C20.14,13.65,19.33,13,18.4,13H13V16z"/><path d="M21.11,18H13v4h6.51c1.28,0,2.23-1.18,1.95-2.43L21.11,18z"/><path d="M4.49,22H11v-4H2.89l-0.35,1.57C2.26,20.82,3.21,22,4.49,22z"/><path d="M12,8L12,8c-0.55,0-1,0.45-1,1v1c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V9C13,8.45,12.55,8,12,8z"/><path d="M18.59,8.62L18.59,8.62c0.39-0.39,0.39-1.02,0-1.41L17.88,6.5c-0.39-0.39-1.02-0.39-1.41,0v0c-0.39,0.39-0.39,1.02,0,1.41 l0.71,0.71C17.57,9.01,18.2,9.01,18.59,8.62z"/><path d="M6.82,8.62l0.71-0.71c0.39-0.39,0.39-1.02,0-1.41l0,0c-0.39-0.39-1.02-0.39-1.41,0L5.41,7.2c-0.39,0.39-0.39,1.02,0,1.41 l0,0C5.8,9.01,6.43,9.01,6.82,8.62z"/><path d="M5,2H4C3.45,2,3,2.45,3,3v0c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v0C6,2.45,5.55,2,5,2z"/><path d="M20,2h-1c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v0C21,2.45,20.55,2,20,2z"/><path d="M12,7c2.76,0,5-2.24,5-5H7C7,4.76,9.24,7,12,7z"/>
        </SvgIcon>
    }
}
