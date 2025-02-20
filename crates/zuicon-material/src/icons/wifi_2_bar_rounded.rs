// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Wifi2BarRounded)]
pub fn wifi_2_bar_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Wifi2BarRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,16c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S13.1,16,12,16z M5.38,14.37c-0.63-0.63-0.59-1.71,0.13-2.24 C7.33,10.79,9.57,10,12,10c2.43,0,4.67,0.79,6.49,2.13c0.72,0.53,0.76,1.6,0.13,2.24c-0.53,0.54-1.37,0.57-1.98,0.12 C15.33,13.55,13.73,13,12,13c-1.73,0-3.33,0.55-4.64,1.49C6.75,14.93,5.91,14.9,5.38,14.37z"/>
        </SvgIcon>
    }
}
