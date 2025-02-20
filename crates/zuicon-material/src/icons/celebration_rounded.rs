// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CelebrationRounded)]
pub fn celebration_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CelebrationRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3.99,21.29l9.04-3.23c1.38-0.49,1.78-2.26,0.74-3.3l-4.53-4.53c-1.04-1.04-2.8-0.64-3.3,0.74l-3.23,9.04 C2.43,20.81,3.19,21.57,3.99,21.29z"/><path d="M15.06,12l5.06-5.06c0.49-0.49,1.28-0.49,1.77,0L21.95,7c0.29,0.29,0.77,0.29,1.06,0l0,0c0.29-0.29,0.29-0.77,0-1.06 l-0.06-0.06c-1.07-1.07-2.82-1.07-3.89,0L14,10.94c-0.29,0.29-0.29,0.77,0,1.06l0,0C14.29,12.29,14.77,12.29,15.06,12z"/><path d="M10.06,6.88L10,6.94C9.71,7.23,9.71,7.71,10,8l0,0c0.29,0.29,0.77,0.29,1.06,0l0.06-0.06c1.07-1.07,1.07-2.82,0-3.89 L11.07,4C10.77,3.7,10.29,3.7,10,4l0,0c-0.29,0.29-0.29,0.77,0,1.06l0.06,0.06C10.54,5.6,10.54,6.4,10.06,6.88z"/><path d="M17.06,11.88L16,12.94c-0.29,0.29-0.29,0.77,0,1.06l0,0c0.29,0.29,0.77,0.29,1.06,0l1.06-1.06 c0.49-0.49,1.28-0.49,1.77,0l1.08,1.08c0.29,0.29,0.77,0.29,1.06,0l0,0c0.29-0.29,0.29-0.77,0-1.06l-1.08-1.08 C19.87,10.81,18.13,10.81,17.06,11.88z"/><path d="M15.06,5.88L12,8.94c-0.29,0.29-0.29,0.77,0,1.06l0,0c0.29,0.29,0.77,0.29,1.06,0l3.06-3.06c1.07-1.07,1.07-2.82,0-3.89 l-1.06-1.06c-0.29-0.29-0.77-0.29-1.06,0l0,0c-0.29,0.29-0.29,0.77,0,1.06l1.06,1.06C15.54,4.6,15.54,5.4,15.06,5.88z"/>
        </SvgIcon>
    }
}
