// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HouseboatRounded)]
pub fn houseboat_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HouseboatRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,17.83c0-0.42-0.27-0.8-0.67-0.94C20.62,16.62,20.21,16,18.67,16c-1.91,0-2.14,1-3.33,1c-1.24,0-1.39-1-3.34-1 s-2.1,1-3.34,1c-1.19,0-1.42-1-3.33-1c-1.54,0-1.95,0.62-2.66,0.88C2.27,17.03,2,17.4,2,17.83c0,0.7,0.69,1.19,1.35,0.95 c0.8-0.29,1.18-0.78,2-0.78c1.19,0,1.42,1,3.33,1c1.95,0,2.08-1,3.32-1s1.37,1,3.32,1c1.91,0,2.14-1,3.33-1c0.83,0,1.21,0.49,2,0.78 C21.31,19.02,22,18.52,22,17.83z M18.91,9.81c0.33-0.45,0.23-1.07-0.22-1.4l-6.1-4.47c-0.35-0.26-0.83-0.26-1.18,0l-6.1,4.47 c-0.45,0.33-0.54,0.95-0.22,1.4c0.33,0.45,0.95,0.54,1.4,0.22L7,9.65V13H5.74c-0.27,0-0.52-0.11-0.71-0.29l-0.66-0.66 c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41l0.66,0.66C4.18,14.68,4.95,15,5.74,15h12.51c0.8,0,1.56-0.32,2.12-0.88 l0.66-0.66c0.39-0.39,0.39-1.02,0-1.41c-0.39-0.39-1.02-0.39-1.41,0l-0.66,0.66C18.78,12.89,18.52,13,18.26,13H17V9.65l0.51,0.37 C17.96,10.35,18.58,10.25,18.91,9.81z M13,13h-2v-2h2V13z"/>
        </SvgIcon>
    }
}
