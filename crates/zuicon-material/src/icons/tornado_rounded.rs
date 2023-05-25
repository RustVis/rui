// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TornadoRounded)]
pub fn tornado_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="TornadoRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M20.11,8l1.16-2c0.77-1.33-0.19-3-1.73-3H4.47c-1.54,0-2.5,1.67-1.73,3l1.16,2H20.11z"/><path d="M7.95,15l2.32,4.01c0.77,1.33,2.69,1.33,3.46,0L16.05,15H7.95z"/>
        </SvgIcon>
    }
}
