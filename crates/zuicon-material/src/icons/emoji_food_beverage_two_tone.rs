// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EmojiFoodBeverageTwoTone)]
pub fn emoji_food_beverage_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EmojiFoodBeverageTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10,6.4l1.81,1.45C11.93,7.94,12,8.09,12,8.24v4.26c0,0.28-0.22,0.5-0.5,0.5h-4C7.22,13,7,12.78,7,12.5 V8.24c0-0.15,0.07-0.3,0.19-0.39L9,6.4V5H6v8c0,1.1,0.9,2,2,2h6c1.1,0,2-0.9,2-2V5h-6V6.4z" opacity=".3"/><path d="M20,3H4v10c0,2.21,1.79,4,4,4h6c2.21,0,4-1.79,4-4v-3h2c1.11,0,2-0.89,2-2V5C22,3.89,21.11,3,20,3z M9.5,7.28l1.5,1.2V12 H8V8.48L9.5,7.28z M16,13c0,1.1-0.9,2-2,2H8c-1.1,0-2-0.9-2-2V5h3v1.4L7.19,7.85C7.07,7.94,7,8.09,7,8.24v4.26 C7,12.78,7.22,13,7.5,13h4c0.28,0,0.5-0.22,0.5-0.5V8.24c0-0.15-0.07-0.3-0.19-0.39L10,6.4V5h6V13z M20,8h-2V5h2V8z"/>
        </SvgIcon>
    }
}
