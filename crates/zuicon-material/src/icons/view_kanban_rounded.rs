// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewKanbanRounded)]
pub fn view_kanban_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewKanbanRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M8,17L8,17c-0.55,0-1-0.45-1-1V8 c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v8C9,16.55,8.55,17,8,17z M12,12L12,12c-0.55,0-1-0.45-1-1V8c0-0.55,0.45-1,1-1h0 c0.55,0,1,0.45,1,1v3C13,11.55,12.55,12,12,12z M16,15L16,15c-0.55,0-1-0.45-1-1V8c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v6 C17,14.55,16.55,15,16,15z"/>
        </SvgIcon>
    }
}
