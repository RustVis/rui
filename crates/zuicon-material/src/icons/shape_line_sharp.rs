// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ShapeLineSharp)]
pub fn shape_line_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ShapeLineSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6,11c2.76,0,5-2.24,5-5S8.76,1,6,1S1,3.24,1,6S3.24,11,6,11z"/><path d="M23,14h-9v9h9V14z"/><path d="M17.71,7.7C18.11,7.89,18.54,8,19,8c1.65,0,3-1.35,3-3s-1.35-3-3-3s-3,1.35-3,3c0,0.46,0.11,0.89,0.3,1.29L6.29,16.3 C5.89,16.11,5.46,16,5,16c-1.65,0-3,1.35-3,3s1.35,3,3,3s3-1.35,3-3c0-0.46-0.11-0.89-0.3-1.29L17.71,7.7z"/>
        </SvgIcon>
    }
}
