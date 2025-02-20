// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MicExternalOffRounded)]
pub fn mic_external_off_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MicExternalOffRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14,6c0-1.24,1.14-2.22,2.42-1.96C17.36,4.24,18,5.13,18,6.09v9.08l2,2V6.16c0-2.08-1.68-4.03-3.76-4.15 C13.92,1.87,12,3.71,12,6v3.17l2,2V6z"/><path d="M10,5c0-1.66-1.34-3-3-3C6.38,2,5.81,2.19,5.33,2.5l4.15,4.15C9.8,6.18,10,5.61,10,5z"/><path d="M1.39,2.81L1.39,2.81C1,3.2,1,3.83,1.39,4.22L5.17,8H5.1c-0.59,0-1.05,0.51-1,1.1l0.85,8.45C4.98,17.81,5.2,18,5.45,18H6 c0,2.34,2.01,4.21,4.39,3.98c2.08-0.2,3.61-2.06,3.61-4.15l0-1l5.78,5.78c0.39,0.39,1.02,0.39,1.41,0l0,0 c0.39-0.39,0.39-1.02,0-1.41L2.81,2.81C2.42,2.42,1.78,2.42,1.39,2.81z M12,17.91c0,0.96-0.64,1.86-1.58,2.05 C9.14,20.22,8,19.24,8,18h0.55c0.26,0,0.47-0.19,0.5-0.45l0.52-5.16L12,14.83V17.91z"/>
        </SvgIcon>
    }
}
