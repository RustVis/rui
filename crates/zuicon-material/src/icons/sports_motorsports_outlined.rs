// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SportsMotorsportsOutlined)]
pub fn sports_motorsports_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SportsMotorsportsOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21.96,11.22C21.57,7.01,17.76,4,13.56,4c-0.19,0-0.38,0.01-0.57,0.02C2,4.74,2,17.2,2,17.2V18c0,1.1,0.9,2,2,2h10 C18.67,20,22.41,15.99,21.96,11.22z M5.26,11.56c0.57-1.29,1.28-2.35,2.14-3.19l3.62,1.53c0.6,0.25,0.98,0.83,0.98,1.48 c0,0.89-0.72,1.61-1.61,1.61H4.72C4.87,12.53,5.04,12.05,5.26,11.56z M18.44,16.04C17.3,17.29,15.68,18,14,18H4v-0.8 c0-0.02,0.01-0.92,0.24-2.2h6.15c1.99,0,3.61-1.62,3.61-3.61c0-1.45-0.87-2.76-2.2-3.32L9.3,7.01c1.1-0.57,2.37-0.9,3.82-0.99 C13.27,6,13.42,6,13.56,6c3.31,0,6.13,2.37,6.41,5.41C20.13,13.13,19.59,14.77,18.44,16.04z"/>
        </SvgIcon>
    }
}
