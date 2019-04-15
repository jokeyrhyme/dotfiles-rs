use crate::lib::{
    ghrafont::GhraFont,
    task::{self, Task},
};

pub fn task() -> Task {
    Task {
        name: String::from("publicsans"),
        sync,
        update,
    }
}

const GHRA_FONT: GhraFont = GhraFont {
    asset_re: r"^public-sans-.*\.zip$",
    font_suffix: ".otf",
    repo: ("uswds", "public-sans"),
};

fn sync() -> task::Result {
    GHRA_FONT.sync()
}

fn update() -> task::Result {
    GHRA_FONT.update()
}
