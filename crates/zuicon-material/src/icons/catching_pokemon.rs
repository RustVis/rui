// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CatchingPokemon)]
pub fn catching_pokemon(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="CatchingPokemon"
            view_box={props.view_box.clone()}
            >
            <path d="M14.5,12c0,1.38-1.12,2.5-2.5,2.5c-1.38,0-2.5-1.12-2.5-2.5s1.12-2.5,2.5-2.5C13.38,9.5,14.5,10.62,14.5,12z M22,12 c0,5.52-4.48,10-10,10C6.48,22,2,17.52,2,12S6.48,2,12,2C17.52,2,22,6.48,22,12z M20,12h-4c0-2.21-1.79-4-4-4c-2.21,0-4,1.79-4,4H4 c0,4.41,3.59,8,8,8C16.41,20,20,16.41,20,12z"/>
        </SvgIcon>
    }
}
