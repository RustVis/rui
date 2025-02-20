// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NestCamWiredStandSharp)]
pub fn nest_cam_wired_stand_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NestCamWiredStandSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,0.85L11.98,1.4C8.95,1.7,6.37,4,6.04,7.03c-0.39,3.57,2.2,6.69,5.68,7.04l1.9,0.19l-0.56,0.85 c-0.88-0.19-1.83-0.18-2.85,0.25C8.21,16.21,7,18.25,7,20.41L7,23h10v-3c0-1.67-0.83-3.15-2.09-4.06l0.97-1.45L18,14.72V0.85z"/>
        </SvgIcon>
    }
}
