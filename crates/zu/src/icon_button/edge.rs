// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use crate::styles::edge::Edge;

#[must_use]
pub const fn css_class(edge: Option<Edge>) -> &'static str {
    match edge {
        Some(Edge::Start) => "ZuIconButton-edgeStart",
        Some(Edge::End) => "ZuIconButton-edgeEnd",
        None => "",
    }
}
