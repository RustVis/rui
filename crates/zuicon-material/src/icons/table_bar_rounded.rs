// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TableBarRounded)]
pub fn table_bar_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="TableBarRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M22,7.5C22,5.57,17.52,4,12,4S2,5.57,2,7.5c0,1.81,3.95,3.31,9,3.48V15H9.35c-0.82,0-1.55,0.5-1.86,1.26l-0.99,2.47 C6.27,19.34,6.71,20,7.37,20h0c0.38,0,0.72-0.23,0.86-0.58L9.2,17h5.6l0.97,2.42c0.14,0.35,0.48,0.58,0.86,0.58h0 c0.66,0,1.11-0.66,0.86-1.27l-0.99-2.47C16.2,15.5,15.46,15,14.65,15H13v-4.02C18.05,10.81,22,9.31,22,7.5z"/>
        </SvgIcon>
    }
}
