// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EnergySavingsLeafOutlined)]
pub fn energy_savings_leaf_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EnergySavingsLeafOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,3C12,3,12,3,12,3c-4.8,0-9,3.86-9,9c0,2.12,0.74,4.07,1.97,5.61L3,19.59L4.41,21l1.97-1.97C7.93,20.26,9.88,21,12,21 c2.3,0,4.61-0.88,6.36-2.64C20.12,16.61,21,14.3,21,12l0-9L12,3z M19,12c0,1.87-0.73,3.63-2.05,4.95C15.63,18.27,13.87,19,12,19 c-3.86,0-7-3.14-7-7c0-1.9,0.74-3.68,2.1-4.99C8.42,5.71,10.16,5,12,5l7,0L19,12z"/><path d="M8.46,12.63l4.05,0.4l-2.44,3.33c-0.11,0.16-0.1,0.38,0.04,0.52c0.15,0.15,0.4,0.16,0.56,0.01l5.16-4.63 c0.33-0.3,0.15-0.85-0.3-0.89l-4.05-0.4l2.44-3.33c0.11-0.16,0.1-0.38-0.04-0.52c-0.15-0.15-0.4-0.16-0.56-0.01l-5.16,4.63 C7.84,12.04,8.02,12.59,8.46,12.63z"/>
        </SvgIcon>
    }
}
