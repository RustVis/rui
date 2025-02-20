// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SwipeUpRounded)]
pub fn swipe_up_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SwipeUpRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8.83,19.1c-0.26-0.6,0.09-1.28,0.73-1.41l3.58-0.71L8.79,7.17c-0.34-0.76,0-1.64,0.76-1.98c0.76-0.34,1.64,0,1.98,0.76 l2.43,5.49l0.84-0.37c0.28-0.13,0.59-0.18,0.9-0.17l4.56,0.21c0.86,0.04,1.6,0.63,1.83,1.45l1.23,4.33 c0.27,0.96-0.2,1.97-1.11,2.37l-5.63,2.49c-0.48,0.21-1.26,0.33-1.76,0.14l-5.45-2.27C9.13,19.53,8.93,19.34,8.83,19.1z M6.75,13.38c0.26-0.26,0.29-0.66,0.09-0.95C5.68,10.74,5,8.7,5,6.5c0-0.88,0.11-1.74,0.32-2.56l1.09,1.09 c0.3,0.3,0.79,0.29,1.08-0.02c0.28-0.3,0.25-0.78-0.04-1.07L5.21,1.71c-0.39-0.39-1.02-0.39-1.41,0L1.53,3.97 c-0.3,0.3-0.29,0.79,0.02,1.08c0.3,0.28,0.78,0.25,1.07-0.04L3.8,3.82C3.6,4.68,3.5,5.58,3.5,6.5c0,2.51,0.77,4.85,2.09,6.77 C5.86,13.66,6.41,13.72,6.75,13.38z"/>
        </SvgIcon>
    }
}
