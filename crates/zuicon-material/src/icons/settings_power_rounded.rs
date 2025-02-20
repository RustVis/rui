// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SettingsPowerRounded)]
pub fn settings_power_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SettingsPowerRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8,24L8,24c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v0C7,23.55,7.45,24,8,24z M12,24L12,24 c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v0C11,23.55,11.45,24,12,24z M12,2c-0.55,0-1,0.45-1,1v8 c0,0.55,0.45,1,1,1s1-0.45,1-1V3C13,2.45,12.55,2,12,2z M15.94,5.06l-0.02,0.02C15.51,5.49,15.56,6.16,16,6.54 c1.51,1.34,2.33,3.43,1.88,5.7c-0.46,2.28-2.29,4.14-4.56,4.62C9.43,17.69,6,14.74,6,11c0-1.78,0.78-3.37,2.01-4.47 c0.43-0.39,0.47-1.04,0.07-1.45L8.06,5.06C7.69,4.69,7.1,4.67,6.7,5.02c-2.01,1.77-3.12,4.53-2.56,7.52 c0.59,3.15,3.11,5.7,6.26,6.31c5.12,0.99,9.6-2.9,9.6-7.85c0-2.38-1.05-4.52-2.71-5.99C16.9,4.67,16.31,4.69,15.94,5.06z M16,24 L16,24c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v0C15,23.55,15.45,24,16,24z"/>
        </SvgIcon>
    }
}
