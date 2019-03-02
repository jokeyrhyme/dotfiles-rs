use crate::lib::{
    ghrafont::GhraFont,
    task::{self, Task},
};

pub fn task() -> Task {
    Task {
        name: String::from("hasklig"),
        sync,
        update,
    }
}

const GHRA_FONT: GhraFont = GhraFont {
    asset_re: r"^Hasklig-.*\.zip$",
    font_suffix: ".otf",
    repo: ("i-tu", "Hasklig"),
};

fn sync() -> task::Result {
    GHRA_FONT.sync()
}

fn update() -> task::Result {
    GHRA_FONT.update()
}
