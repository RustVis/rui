// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FeedSharp)]
pub fn feed_sharp(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="FeedSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M16,3H3v18h18V8L16,3z M7,7h5v2H7V7z M17,17H7v-2h10V17z M17,13H7v-2h10V13z M15,9V5l4,4H15z"/>
        </SvgIcon>
    }
}
