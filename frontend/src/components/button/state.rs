use std::{
    rc::Rc,
    cell::RefCell,
    borrow::Cow,
};
use futures_signals::signal::{SignalExt, Mutable};
use super::styles;
use crate::{components::image::Image, utils::prelude::*};

pub struct Button {
    pub style: ButtonStyle,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ButtonStyle {
    Color(ButtonColor, Cow<'static, str>),
    Image(Image)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonColor {
    Blue,
    Red,
}


impl Button {
    pub fn new_color(color: ButtonColor, text: impl Into<Cow<'static, str>>) -> Self {
        Self {
            style: ButtonStyle::Color(color, text.into()),
        }
    }
    pub fn new_image(image: Image) -> Self {
        Self {
            style: ButtonStyle::Image(image),
        }
    }
}

impl ButtonColor {
    pub fn get_class(&self) -> &'static str {

        match self {
            Self::Blue => &*styles::COLOR_BLUE,
            Self::Red => &*styles::COLOR_RED,
        }
    }
}