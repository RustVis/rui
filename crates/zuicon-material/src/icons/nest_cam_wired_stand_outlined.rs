// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NestCamWiredStandOutlined)]
pub fn nest_cam_wired_stand_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NestCamWiredStandOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16,1c-0.15,0,0.11-0.02-4.28,0.42C8.47,1.75,6,4.48,6,7.75s2.47,6,5.72,6.33l1.9,0.19l-0.56,0.85 C12.71,15.04,12.36,15,12,15c-2.76,0-5,2.24-5,5v2c0,0.55,0.45,1,1,1h8c0.55,0,1-0.45,1-1v-2c0-1.67-0.83-3.15-2.09-4.06l0.97-1.45 c0.04,0,0.09,0.01,0.13,0.01c1.09,0,2-0.89,2-2V3C18,1.89,17.09,1,16,1z M15,21H9v-1c0-1.65,1.35-3,3-3c1.65,0,3,1.35,3,3V21z M8,7.75c0-2.25,1.69-4.11,3.92-4.34L16,3h0l0.03,9.5l-4.11-0.42C9.69,11.86,8,10,8,7.75z"/>
        </SvgIcon>
    }
}
