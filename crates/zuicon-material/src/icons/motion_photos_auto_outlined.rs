// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MotionPhotosAutoOutlined)]
pub fn motion_photos_auto_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MotionPhotosAutoOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2.88,7.88l1.54,1.54C4.15,10.23,4,11.1,4,12c0,4.41,3.59,8,8,8s8-3.59,8-8s-3.59-8-8-8c-0.9,0-1.77,0.15-2.58,0.42 L7.89,2.89C9.15,2.32,10.54,2,12,2c5.52,0,10,4.48,10,10s-4.48,10-10,10S2,17.52,2,12C2,10.53,2.32,9.14,2.88,7.88z M7,5.5 C7,6.33,6.33,7,5.5,7S4,6.33,4,5.5S4.67,4,5.5,4S7,4.67,7,5.5z M12.03,8.99h-0.07l-1.16,3.31h2.39L12.03,8.99z M11.29,7.5h1.43 l3.01,8h-1.39l-0.72-2.04h-3.23L9.66,15.5H8.28L11.29,7.5z"/>
        </SvgIcon>
    }
}
