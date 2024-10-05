use bevy::prelude::*;
use bevy_ui_dsl::*;

use super::classes::*;

pub fn w_root(
    class: impl Class<NodeBundle>,
    assets: &AssetServer,
    commands: &mut Commands,
    extras: impl Bundle,
    children: impl FnOnce(&mut UiChildBuilder),
) -> Entity {
    let mut bundle = NodeBundle::default();

    c_root.apply(&mut bundle);
    class.apply(&mut bundle);

    commands
        .spawn((bundle, extras))
        .with_children(|builder| children(&mut UiChildBuilder::new(builder, assets)))
        .id()
}

pub fn w_text_button(
    txt: impl Into<String>,
    class: impl AssetClass<ButtonBundle>,
    text_style: impl AssetClass<TextStyle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder,
) -> Entity {
    let classes = (c_text_black, c_text_kose, c_text_h2, text_style);
    w_button(class, extras, parent, |p| {
        w_text(txt, (), classes, (), p);
    })
}

pub fn w_text(
    text: impl Into<String>,
    class: impl AssetClass<TextBundle>,
    text_class: impl AssetClass<TextStyle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder,
) -> Entity {
    let assets = parent.assets();
    let mut bundle = TextBundle::default();
    c_text.apply(assets, &mut bundle);
    class.apply(assets, &mut bundle);

    let sections = &mut bundle.text.sections;

    let mut style = TextStyle::default();
    let classes = (c_text_white, c_text_kose, c_text_h1, text_class);
    classes.apply(assets, &mut style);

    sections.push(TextSection {
        value: text.into(),
        style,
    });

    parent.spawn((bundle, extras)).id()
}

pub fn w_button(
    class: impl AssetClass<ButtonBundle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder,
    children: impl FnOnce(&mut UiChildBuilder),
) -> Entity {
    let assets = parent.assets();
    let mut bundle = ButtonBundle::default();

    c_button.apply(assets, &mut bundle);
    class.apply(assets, &mut bundle);

    parent.spawn((bundle, extras)).with_children(children).id()
}
