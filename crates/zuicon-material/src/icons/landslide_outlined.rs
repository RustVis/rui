// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LandslideOutlined)]
pub fn landslide_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LandslideOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M11,12L8,8H2v14h20l-6-8L11,12z M12.53,14.77L6,16.95l-2-0.67v-1.89l2,0.67l3.95-1.32L12.53,14.77z M7,10l1.57,2.09 L6,12.95l-2-0.67V10H7z M4,20v-1.61l2,0.67l9.03-3.01L18,20H4z"/><path d="M17,6V1l-5-1L9,2v4l3,2L17,6z M11,3.07l1.42-0.95L15,2.64v2.01l-2.77,1.11L11,4.93V3.07z"/><path d="M18.5,7L16,9v3l2.5,2l4.5-2V8L18.5,7z M21,10.7l-2.2,0.98L18,11.04V9.96l1-0.8l2,0.44V10.7z"/>
        </SvgIcon>
    }
}
