// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SportsBarRounded)]
pub fn sports_bar_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SportsBarRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,9h-1.56c0.33-0.55,0.53-1.18,0.55-1.86c0.04-1.03-0.43-1.99-1.16-2.71c-1.54-1.54-2.74-1.56-3.82-1.29 C12.2,2.45,11.16,2.02,10,2.02c-1.89,0-3.51,1.11-4.27,2.71C4.15,5.26,3,6.74,3,8.5c0,1.86,1.28,3.41,3,3.86L6,19c0,1.1,0.9,2,2,2h7 c1.1,0,2-0.9,2-2v0h2c1.1,0,2-0.9,2-2v-6C21,9.9,20.1,9,19,9z M7,10.5c-1.1,0-2-0.9-2-2c0-0.85,0.55-1.6,1.37-1.88l0.8-0.27 l0.36-0.76C8,4.62,8.94,4.02,10,4.02c0.79,0,1.39,0.35,1.74,0.65l0.78,0.65c0,0,0.64-0.32,1.47-0.32c1.1,0,2,0.9,2,2c0,0-3,0-3,0 C9.67,7,9.15,10.5,7,10.5z M19,17h-2v-6h2V17z"/>
        </SvgIcon>
    }
}
