// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::prelude::*;
use yew_router::prelude::Routable;

use crate::views::alert_page::AlertPage;
use crate::views::avatar_page::AvatarPage;
use crate::views::badge_page::BadgePage;
use crate::views::box_page::BoxPage;
use crate::views::container_page::ContainerPage;
use crate::views::divider_page::DividerPage;
use crate::views::home_page::HomePage;
use crate::views::icons_page::IconsPage;
use crate::views::paper_page::PaperPage;
use crate::views::progress_page::ProgressPage;
use crate::views::skeleton_page::SkeletonPage;
use crate::views::stack_page::StackPage;
use crate::views::typography_page::TypographyPage;
use crate::views::material_icon_page::MaterialIconPage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/alert")]
    Alert,

    #[at("/avatar")]
    Avatar,

    #[at("/badge")]
    Badge,

    #[at("/box")]
    Box,

    #[at("/container")]
    Container,

    #[at("/divider")]
    Divider,

    #[at("/material-icon")]
    MaterialIcon,

    #[at("/icons")]
    Icons,

    #[at("/paper")]
    Paper,

    #[at("/progress")]
    Progress,

    #[at("/skeleton")]
    Skeleton,

    #[at("/stack")]
    Stack,

    #[at("/typography")]
    Typography,
}

#[must_use]
#[allow(clippy::cognitive_complexity)]
#[allow(clippy::let_unit_value)]
pub fn switch_route(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<HomePage />},
        Route::Alert => html! {<AlertPage />},
        Route::Avatar => html! {<AvatarPage />},
        Route::Badge => html! {<BadgePage />},
        Route::Box => html! {<BoxPage />},
        Route::Container => html! {<ContainerPage />},
        Route::Divider => html! {<DividerPage />},
        Route::Icons => html! {<IconsPage />},
        Route::MaterialIcon => html!{ <MaterialIconPage />},
        Route::Paper => html! {<PaperPage />},
        Route::Progress => html! {<ProgressPage />},
        Route::Skeleton => html! {<SkeletonPage />},
        Route::Stack => html! {<StackPage />},
        Route::Typography => html! {<TypographyPage />},
    }
}
