// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelVariant {
    Filled,
    Outlined,
    Standard,
}

impl Default for LabelVariant {
    fn default() -> Self {
        Self::Standard
    }
}

impl LabelVariant {
    #[inline]
    #[must_use]
    pub fn is_contained(self) -> bool {
        self == Self::Filled || self == Self::Outlined
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimpleLabelVariant {
    Filled,
    Outlined,
}

impl Default for SimpleLabelVariant {
    fn default() -> Self {
        Self::Filled
    }
}
