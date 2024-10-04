use bevy::prelude::*;

pub fn c_root(n: &mut NodeBundle) {
    let s = &mut n.style;
    s.width = Val::Percent(100.);
    s.height = Val::Percent(100.);
    s.align_items = AlignItems::Center;
    s.justify_content = JustifyContent::Center;
    s.flex_direction = FlexDirection::Column;
    s.row_gap = Val::Px(25.);
}

pub fn c_text(_: &AssetServer, t: &mut TextBundle) {
    t.style.margin = UiRect::all(Val::Px(10.));
}

pub fn c_text_black(_: &AssetServer, s: &mut TextStyle) {
    s.color = Color::BLACK.into();
}

pub fn c_text_white(_: &AssetServer, s: &mut TextStyle) {
    s.color = Color::WHITE.into();
}

pub fn c_text_kose(a: &AssetServer, s: &mut TextStyle) {
    s.font = a.load("Kosefont.ttf").into();
}

pub fn c_text_h1(_: &AssetServer, s: &mut TextStyle) {
    s.font_size = 128.;
}

pub fn c_text_h2(_: &AssetServer, s: &mut TextStyle) {
    s.font_size = 64.;
}

pub fn c_button(_: &AssetServer, b: &mut ButtonBundle) {
    let s = &mut b.style;

    s.width = Val::Px(300.);
    s.height = Val::Px(80.);
    s.justify_content = JustifyContent::Center;
    s.align_items = AlignItems::Center;
    s.border = UiRect::all(Val::Px(1.0));
    b.background_color = Color::WHITE.into();
    b.border_color = Color::NONE.into();
}
