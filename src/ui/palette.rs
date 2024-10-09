use bevy::prelude::*;

pub struct ColorPalettePlugin;

impl Plugin for ColorPalettePlugin {
    fn build(&self, app: &mut App) {
        let expect_color = |hex_str| Color::Srgba(Srgba::hex(hex_str).expect("Invalid hex code."));

        app.insert_resource(ColorPalette {
            black: expect_color("#0B0F10"),
            white: expect_color("#C5C8C9"),
            red: expect_color("#D4515B"),
            yellow: expect_color("#FFE8AF"),
            orange: expect_color("#E8C5AC"),
            blue: expect_color("#618592"),
            pink: expect_color("#FFA1A8"),
            cyan: expect_color("#99B9D8"),
        });
    }
}

#[allow(unused)]
#[derive(Resource)]
pub struct ColorPalette {
    pub black: Color,
    pub white: Color,
    pub red: Color,
    pub yellow: Color,
    pub orange: Color,
    pub blue: Color,
    pub pink: Color,
    pub cyan: Color,
}
