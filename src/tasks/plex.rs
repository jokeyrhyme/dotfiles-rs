use crate::lib::{
    ghrafont::GhraFont,
    task::{self, Status, Task},
};

pub fn task() -> Task {
    Task {
        name: String::from("plex"),
        sync,
        update,
    }
}

const GHRA_FONT: GhraFont = GhraFont {
    asset_re: r"^OpenType\.zip$",
    font_suffix: ".otf",
    repo: ("IBM", "plex"),
};

fn sync() -> task::Result {
    GHRA_FONT.sync()
}

fn update(sync: Status) -> task::Result {
    GHRA_FONT.update(sync)
}
