// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LocalFireDepartmentOutlined)]
pub fn local_fire_department_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LocalFireDepartmentOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16,6l-0.44,0.55c-0.42,0.52-0.98,0.75-1.54,0.75C13,7.3,12,6.52,12,5.3V2c0,0-8,4-8,11c0,4.42,3.58,8,8,8s8-3.58,8-8 C20,10.04,18.39,7.38,16,6z M12,19c-1.1,0-2-0.87-2-1.94c0-0.51,0.2-0.99,0.58-1.36l1.42-1.4l1.43,1.4 C13.8,16.07,14,16.55,14,17.06C14,18.13,13.1,19,12,19z M15.96,17.5L15.96,17.5c0.04-0.36,0.22-1.89-1.13-3.22l0,0L12,11.5 l-2.83,2.78l0,0c-1.36,1.34-1.17,2.88-1.13,3.22C6.79,16.4,6,14.79,6,13c0-3.16,2.13-5.65,4.03-7.25c0.23,1.99,1.93,3.55,3.99,3.55 c0.78,0,1.54-0.23,2.18-0.66C17.34,9.78,18,11.35,18,13C18,14.79,17.21,16.4,15.96,17.5z"/>
        </SvgIcon>
    }
}
