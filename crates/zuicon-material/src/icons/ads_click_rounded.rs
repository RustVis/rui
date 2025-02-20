// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AdsClickRounded)]
pub fn ads_click_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AdsClickRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M11.71,17.99C8.53,17.84,6,15.22,6,12c0-3.31,2.69-6,6-6c3.22,0,5.84,2.53,5.99,5.71l-2.1-0.63C15.48,9.31,13.89,8,12,8 c-2.21,0-4,1.79-4,4c0,1.89,1.31,3.48,3.08,3.89L11.71,17.99z M22,12c0,0.3-0.01,0.6-0.04,0.9l-1.97-0.59C20,12.21,20,12.1,20,12 c0-4.42-3.58-8-8-8s-8,3.58-8,8s3.58,8,8,8c0.1,0,0.21,0,0.31-0.01l0.59,1.97C12.6,21.99,12.3,22,12,22C6.48,22,2,17.52,2,12 C2,6.48,6.48,2,12,2S22,6.48,22,12z M18.23,16.26l2.27-0.76c0.46-0.15,0.45-0.81-0.01-0.95l-7.6-2.28 c-0.38-0.11-0.74,0.24-0.62,0.62l2.28,7.6c0.14,0.47,0.8,0.48,0.95,0.01l0.76-2.27l3.91,3.91c0.2,0.2,0.51,0.2,0.71,0l1.27-1.27 c0.2-0.2,0.2-0.51,0-0.71L18.23,16.26z"/>
        </SvgIcon>
    }
}
