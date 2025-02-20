// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ScreenLockRotationRounded)]
pub fn screen_lock_rotation_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ScreenLockRotationRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M20.41,11.36l-0.35-0.35c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41L19,12.77l-4.24,4.24L6.98,9.23 l4.24-4.24l0.35,0.35c0.39,0.39,1.02,0.39,1.41,0c0.39-0.39,0.39-1.02,0-1.41l-0.35-0.36c-0.79-0.79-2.03-0.79-2.82,0L5.57,7.82 c-0.78,0.78-0.78,2.05,0,2.83l7.78,7.78c0.79,0.79,2.03,0.79,2.82,0l4.24-4.24C21.2,13.41,21.2,12.14,20.41,11.36z"/><path d="M10.85,17.85C10.54,17.54,10,17.76,10,18.21v1.53c-3.17-0.82-5.59-3.54-5.95-6.86C3.99,12.37,3.56,12,3.06,12 c-0.6,0-1.07,0.53-1,1.12C2.62,18.11,6.87,22,12,22c0.59,0,1.17-0.06,1.73-0.16c0.4-0.07,0.55-0.56,0.27-0.85L10.85,17.85z"/><path d="M16,9h4c0.55,0,1-0.45,1-1V5c0-0.55-0.45-1-1-1V3.11c0-1-0.68-1.92-1.66-2.08C17.08,0.82,16,1.79,16,3v1 c-0.55,0-1,0.45-1,1v3C15,8.55,15.45,9,16,9z M17,3c0-0.55,0.45-1,1-1s1,0.45,1,1v1h-2V3z"/>
        </SvgIcon>
    }
}
