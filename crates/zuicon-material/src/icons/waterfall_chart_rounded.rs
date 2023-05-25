// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WaterfallChartRounded)]
pub fn waterfall_chart_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WaterfallChartRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M19.5,4L19.5,4C20.33,4,21,4.67,21,5.5v13c0,0.83-0.67,1.5-1.5,1.5h0c-0.83,0-1.5-0.67-1.5-1.5v-13 C18,4.67,18.67,4,19.5,4z M4.5,13L4.5,13C5.33,13,6,13.67,6,14.5v4C6,19.33,5.33,20,4.5,20h0C3.67,20,3,19.33,3,18.5v-4 C3,13.67,3.67,13,4.5,13z M15.5,4L15.5,4C16.33,4,17,4.67,17,5.5v0C17,6.33,16.33,7,15.5,7h0C14.67,7,14,6.33,14,5.5v0 C14,4.67,14.67,4,15.5,4z M11.5,5L11.5,5C12.33,5,13,5.67,13,6.5v1C13,8.33,12.33,9,11.5,9h0C10.67,9,10,8.33,10,7.5v-1 C10,5.67,10.67,5,11.5,5z M8.5,10L8.5,10c0.83,0,1.5,0.67,1.5,1.5v1c0,0.83-0.67,1.5-1.5,1.5h0C7.67,14,7,13.33,7,12.5v-1 C7,10.67,7.67,10,8.5,10z"/>
        </SvgIcon>
    }
}
