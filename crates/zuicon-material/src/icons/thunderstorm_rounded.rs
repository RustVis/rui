// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ThunderstormRounded)]
pub fn thunderstorm_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ThunderstormRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17.92,7.02C17.45,4.18,14.97,2,12,2C9.82,2,7.83,3.18,6.78,5.06C4.09,5.41,2,7.74,2,10.5C2,13.53,4.47,16,7.5,16h10 c2.48,0,4.5-2.02,4.5-4.5C22,9.16,20.21,7.23,17.92,7.02z"/><path d="M15.95,20.11l-0.84-0.42l0.9-1.03c0.36-0.42,0.32-1.05-0.09-1.41c-0.42-0.36-1.05-0.32-1.41,0.09l-1.75,2 c-0.2,0.23-0.29,0.55-0.23,0.85c0.06,0.3,0.26,0.56,0.53,0.7l0.84,0.42L13,22.34c-0.36,0.42-0.32,1.05,0.09,1.41 c0.19,0.17,0.42,0.25,0.66,0.25c0.28,0,0.55-0.12,0.75-0.34l1.75-2c0.2-0.23,0.29-0.55,0.23-0.85 C16.42,20.5,16.22,20.24,15.95,20.11z"/><path d="M9.95,20.11L9.1,19.68l0.9-1.03c0.36-0.42,0.32-1.05-0.09-1.41c-0.42-0.36-1.05-0.32-1.41,0.09l-1.75,2 c-0.2,0.23-0.29,0.55-0.23,0.85c0.06,0.3,0.26,0.56,0.53,0.7l0.84,0.42L7,22.34c-0.36,0.42-0.32,1.05,0.09,1.41 C7.28,23.92,7.52,24,7.75,24c0.28,0,0.55-0.12,0.75-0.34l1.75-2c0.2-0.23,0.29-0.55,0.23-0.85C10.42,20.5,10.22,20.24,9.95,20.11z"/>
        </SvgIcon>
    }
}
