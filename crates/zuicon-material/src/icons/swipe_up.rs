// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SwipeUp)]
pub fn swipe_up(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SwipeUp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2.06,5.56L1,4.5L4.5,1L8,4.5L6.94,5.56L5.32,3.94C5.11,4.76,5,5.62,5,6.5c0,2.42,0.82,4.65,2.2,6.43L6.13,14 C4.49,11.95,3.5,9.34,3.5,6.5c0-0.92,0.1-1.82,0.3-2.68L2.06,5.56z M13.85,11.62l-2.68-5.37c-0.37-0.74-1.27-1.04-2.01-0.67 C8.41,5.96,8.11,6.86,8.48,7.6l4.81,9.6L10.05,18c-0.33,0.09-0.59,0.33-0.7,0.66L9,19.78l6.19,2.25c0.5,0.17,1.28,0.02,1.75-0.22 l5.51-2.75c0.89-0.45,1.32-1.48,1-2.42l-1.43-4.27c-0.27-0.82-1.04-1.37-1.9-1.37h-4.56c-0.31,0-0.62,0.07-0.89,0.21L13.85,11.62"/>
        </SvgIcon>
    }
}
