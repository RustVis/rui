// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FollowTheSignsRounded)]
pub fn follow_the_signs_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FollowTheSignsRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9.5,5.5c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2S8.4,5.5,9.5,5.5z M5.75,8.9L3.23,21.81C3.11,22.43,3.58,23,4.21,23H4.3 c0.47,0,0.88-0.33,0.98-0.79L6.85,15L9,17v5c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-6.14c0-0.27-0.11-0.52-0.29-0.71L8.95,13.4 l0.6-3c1.07,1.32,2.58,2.23,4.31,2.51c0.6,0.1,1.14-0.39,1.14-1v0c0-0.49-0.36-0.9-0.84-0.98c-1.49-0.25-2.75-1.15-3.51-2.38 L9.7,6.95C9.35,6.35,8.7,6,8,6C7.75,6,7.5,6.05,7.25,6.15l-4.63,1.9C2.25,8.2,2,8.57,2,8.97V12c0,0.55,0.45,1,1,1h0 c0.55,0,1-0.45,1-1V9.65L5.75,8.9 M21,2h-7c-0.55,0-1,0.45-1,1v5c0,0.55,0.45,1,1,1h2.75v13.25c0,0.41,0.34,0.75,0.75,0.75 s0.75-0.34,0.75-0.75V9H21c0.55,0,1-0.45,1-1V3C22,2.45,21.55,2,21,2z M20.15,5.85l-1.28,1.29c-0.31,0.32-0.85,0.09-0.85-0.35V6.25 h-2.76c-0.41,0-0.75-0.34-0.75-0.75s0.34-0.75,0.75-0.75h2.76V4.21c0-0.45,0.54-0.67,0.85-0.35l1.28,1.29 C20.34,5.34,20.34,5.66,20.15,5.85z"/>
        </SvgIcon>
    }
}
