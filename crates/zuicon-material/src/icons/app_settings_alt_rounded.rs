// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AppSettingsAltRounded)]
pub fn app_settings_alt_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AppSettingsAltRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M17,18H7V6h10v1h2V3c0-1.1-0.9-2-2-2L7,1.01C5.9,1.01,5,1.9,5,3v18c0,1.1,0.9,2,2,2h10c1.1,0,2-0.9,2-2v-4h-2V18z M21,12 c0-0.13-0.02-0.26-0.04-0.39l0.64-0.48c0.2-0.15,0.26-0.44,0.13-0.66l-0.57-0.96c-0.13-0.21-0.39-0.3-0.62-0.2l-0.72,0.3 c-0.2-0.15-0.42-0.29-0.65-0.39l-0.1-0.77c-0.03-0.25-0.24-0.43-0.49-0.44l-1.12-0.02c-0.26,0-0.47,0.18-0.5,0.44l-0.1,0.79 c-0.24,0.1-0.45,0.23-0.65,0.39l-0.72-0.3c-0.23-0.1-0.5-0.01-0.62,0.2l-0.57,0.96c-0.13,0.22-0.08,0.5,0.13,0.66l0.64,0.48 C15.02,11.74,15,11.87,15,12c0,0.13,0.02,0.25,0.04,0.37l-0.64,0.49c-0.2,0.15-0.26,0.43-0.13,0.65l0.56,0.97 c0.13,0.22,0.39,0.31,0.63,0.21l0.73-0.31c0.2,0.16,0.42,0.3,0.67,0.4l0.1,0.77c0.03,0.25,0.24,0.44,0.5,0.44h1.12 c0.25,0,0.46-0.19,0.5-0.44l0.1-0.77c0.24-0.1,0.46-0.24,0.67-0.4l0.73,0.31c0.23,0.1,0.5,0.01,0.63-0.21l0.56-0.97 c0.13-0.22,0.07-0.5-0.13-0.65l-0.64-0.49C20.98,12.25,21,12.13,21,12z M18,13.5c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5 s1.5,0.67,1.5,1.5S18.83,13.5,18,13.5z"/>
        </SvgIcon>
    }
}
