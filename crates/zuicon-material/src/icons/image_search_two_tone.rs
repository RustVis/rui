// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ImageSearchTwoTone)]
pub fn image_search_two_tone(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="ImageSearchTwoTone"
            view_box={props.view_box.clone()}
            >
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M17.7 11.53c-.7.31-1.45.47-2.21.47C12.46 12 10 9.53 10 6.5c0-.17.01-.34.03-.5H4v14h14v-8.17l-.3-.3zM5.5 18l2.75-3.53 1.96 2.36 2.75-3.54L16.5 18h-11z" opacity=".3"/><path d="M10.21 16.83l-1.96-2.36L5.5 18h11l-3.54-4.71zM20 6.5C20 4.01 17.99 2 15.5 2S11 4.01 11 6.5s2.01 4.5 4.49 4.5c.88 0 1.7-.26 2.39-.7L21 13.42 22.42 12 19.3 8.89c.44-.7.7-1.51.7-2.39zM15.5 9C14.12 9 13 7.88 13 6.5S14.12 4 15.5 4 18 5.12 18 6.5 16.88 9 15.5 9zM18 20H4V6h6.03c.06-.72.27-1.39.58-2H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-6.17l-2-2V20z"/>
        </SvgIcon>
    }
}
