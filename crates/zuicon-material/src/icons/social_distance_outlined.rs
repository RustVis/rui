// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SocialDistanceOutlined)]
pub fn social_distance_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SocialDistanceOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,5c0-1.1,0.9-2,2-2s2,0.9,2,2c0,1.1-0.9,2-2,2S4,6.1,4,5z M8.78,8.58C7.93,8.21,6.99,8,6,8S4.07,8.21,3.22,8.58 C2.48,8.9,2,9.62,2,10.43L2,11h8l0-0.57C10,9.62,9.52,8.9,8.78,8.58z M18,7c1.1,0,2-0.9,2-2c0-1.1-0.9-2-2-2s-2,0.9-2,2 C16,6.1,16.9,7,18,7z M20.78,8.58C19.93,8.21,18.99,8,18,8c-0.99,0-1.93,0.21-2.78,0.58C14.48,8.9,14,9.62,14,10.43L14,11h8l0-0.57 C22,9.62,21.52,8.9,20.78,8.58z M18.01,13.01l-1.41,1.41L18.17,16H5.83l1.58-1.59l-1.41-1.41L2,17l3.99,3.99l1.41-1.41L5.83,18 h12.34l-1.58,1.58l1.41,1.41L22,17L18.01,13.01z"/>
        </SvgIcon>
    }
}
