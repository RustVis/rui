// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SwipeUpSharp)]
pub fn swipe_up_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SwipeUpSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2.06,5.56L1,4.5L4.5,1L8,4.5L6.94,5.56L5.32,3.94C5.11,4.76,5,5.62,5,6.5c0,2.42,0.82,4.65,2.2,6.43L6.13,14 C4.49,11.95,3.5,9.34,3.5,6.5c0-0.92,0.1-1.82,0.3-2.68L2.06,5.56z M21.71,11.18l2.09,7.39l-8.23,3.65l-6.84-2.85l0.61-1.62 l3.8-0.75L8.79,7.17c-0.34-0.76,0-1.64,0.76-1.98c0.76-0.34,1.64,0,1.98,0.76l2.43,5.49l1.26-0.56L21.71,11.18z"/>
        </SvgIcon>
    }
}
