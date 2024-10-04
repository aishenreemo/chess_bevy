use bevy::prelude::*;

pub fn c_root(b: &mut NodeBundle) {
    b.style.width = Val::Percent(100.);
    b.style.height = Val::Percent(100.);
    b.style.align_items = AlignItems::Center;
    b.style.justify_content = JustifyContent::Center;
    b.style.flex_direction = FlexDirection::Column;
    b.style.row_gap = Val::Px(30.);
}

pub fn c_text(_a: &AssetServer, b: &mut TextBundle) {
    b.style.margin = UiRect::all(Val::Px(10.));
}

pub fn c_text_black(_a: &AssetServer, b: &mut TextStyle) {
    b.color = Color::BLACK.into();
}

pub fn c_text_white(_a: &AssetServer, b: &mut TextStyle) {
    b.color = Color::WHITE.into();
}

pub fn c_text_h1(a: &AssetServer, b: &mut TextStyle) {
    b.font = a.load("Kosefont.ttf").into();
    b.font_size = 128.;
}

pub fn c_text_h2(a: &AssetServer, b: &mut TextStyle) {
    b.font = a.load("Kosefont.ttf").into();
    b.font_size = 64.;
}

pub fn c_button(_a: &AssetServer, b: &mut ButtonBundle) {
    let s = &mut b.style;

    s.width = Val::Px(300.);
    s.height = Val::Px(80.);
    s.justify_content = JustifyContent::Center;
    s.align_items = AlignItems::Center;
    s.border = UiRect::all(Val::Px(1.0));
    b.background_color = Color::WHITE.into();
    b.border_color = Color::NONE.into();
}
