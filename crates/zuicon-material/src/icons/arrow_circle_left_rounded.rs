// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ArrowCircleLeftRounded)]
pub fn arrow_circle_left_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ArrowCircleLeftRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,12c0,5.52,4.48,10,10,10c5.52,0,10-4.48,10-10S17.52,2,12,2C6.48,2,2,6.48,2,12z M12,9.21L12,11l3,0c0.55,0,1,0.45,1,1 s-0.45,1-1,1l-3,0l0,1.79c0,0.45-0.54,0.67-0.85,0.35l-2.79-2.79c-0.2-0.2-0.2-0.51,0-0.71l2.79-2.79C11.46,8.54,12,8.76,12,9.21z"/>
        </SvgIcon>
    }
}
