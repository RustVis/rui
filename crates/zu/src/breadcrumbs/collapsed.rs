// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Html, Properties};

use crate::button_base::ButtonBase;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub collapsed_icon: Html,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(BreadcrumbsCollapsed)]
pub fn breadcrumbs_collapsed(_props: &Props) -> Html {
    html! {
        <li>
            <ButtonBase
                classes="ZuBreadcrumbCollapsed-button"
                style={props.style}
                focus_ripple={true}>
                <MoreHorizonIcon>
                    {props.collapsed_icon.clone()}
                </MoreHorizonIcon>
            </ButtonBase>
        </li>
    }
}
