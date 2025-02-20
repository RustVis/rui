// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RoundaboutRightRounded)]
pub fn roundabout_right_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RoundaboutRightRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8,21c0.55,0,1-0.45,1-1l0-5.09c0-0.98-0.71-1.8-1.67-1.97C5.44,12.63,4,10.98,4,9c0-2.21,1.79-4,4-4 c1.98,0,3.63,1.44,3.94,3.33C12.11,9.29,12.93,10,13.91,10l4.26,0l-0.88,0.88c-0.39,0.39-0.39,1.02,0,1.41 c0.39,0.39,1.02,0.39,1.41,0l2.59-2.59c0.39-0.39,0.39-1.02,0-1.41l-2.59-2.59c-0.39-0.39-1.02-0.39-1.41,0 c-0.39,0.39-0.39,1.02,0,1.41L18.17,8l-4.25,0C13.44,5.16,10.97,3,8,3C4.69,3,2,5.69,2,9c0,2.97,2.16,5.44,5,5.92L7,20 C7,20.55,7.45,21,8,21z"/>
        </SvgIcon>
    }
}
