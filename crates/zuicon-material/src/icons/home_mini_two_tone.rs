// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HomeMiniTwoTone)]
pub fn home_mini_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HomeMiniTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,7c-7.91,0-8,4.8-8,5h16C19.99,11.51,19.64,7,12,7z" opacity=".3"/><path d="M9.14,17h5.72c2.1,0,3.92-1.24,4.71-3H4.42C5.22,15.76,7.04,17,9.14,17z" opacity=".3"/><path d="M12,5C4.19,5,2,9.48,2,12c0,3.86,3.13,7,6.99,7h6.02c2.69,0,6.99-2.08,6.99-7C22,12,22,5,12,5z M14.86,17H9.14 c-2.1,0-3.92-1.24-4.71-3h15.15C18.78,15.76,16.96,17,14.86,17z M4,12c0-0.2,0.09-5,8-5c7.64,0,7.99,4.51,8,5H4z"/>
        </SvgIcon>
    }
}
