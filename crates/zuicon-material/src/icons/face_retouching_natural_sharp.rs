// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FaceRetouchingNaturalSharp)]
pub fn face_retouching_natural_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FaceRetouchingNaturalSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19.85,10.59C20.79,15.4,17.01,20,12,20c-4.41,0-8-3.59-8-8c0-0.39,3.87-1.12,5.74-5.69c3.42,4.19,8.07,3.73,9.09,3.59 l-1.48-3.25L12.63,4.5l3.5-1.59C9.51-0.14,2,4.77,2,12c0,5.52,4.48,10,10,10c7.21,0,12.12-7.45,9.1-14.13L19.85,10.59z"/><path d="M20.6,5.6L19.5,8l-1.1-2.4L16,4.5l2.4-1.1L19.5,1l1.1,2.4L23,4.5L20.6,5.6z"/>
        </SvgIcon>
    }
}
