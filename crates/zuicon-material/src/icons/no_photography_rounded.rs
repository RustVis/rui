// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NoPhotographyRounded)]
pub fn no_photography_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NoPhotographyRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10.94,8.12L7.48,4.66L8.4,3.65C8.78,3.24,9.32,3,9.88,3h4.24c0.56,0,1.1,0.24,1.47,0.65L16.83,5H20c1.1,0,2,0.9,2,2v12 c0,0.05-0.01,0.1-0.02,0.16l-5.1-5.1C16.96,13.71,17,13.36,17,13c0-2.76-2.24-5-5-5C11.64,8,11.29,8.04,10.94,8.12z M19.78,22.61 L18.17,21H4c-1.1,0-2-0.9-2-2V7c0-0.59,0.27-1.12,0.68-1.49L1.39,4.22C1,3.83,1,3.2,1.39,2.81l0,0c0.39-0.39,1.02-0.39,1.41,0 l18.38,18.38c0.39,0.39,0.39,1.02,0,1.41l0,0C20.8,23,20.17,23,19.78,22.61z M14.49,17.32l-1.5-1.5C12.67,15.92,12.35,16,12,16 c-1.66,0-3-1.34-3-3c0-0.35,0.08-0.67,0.19-0.98l-1.5-1.5C7.25,11.24,7,12.09,7,13c0,2.76,2.24,5,5,5 C12.91,18,13.76,17.75,14.49,17.32z"/>
        </SvgIcon>
    }
}
