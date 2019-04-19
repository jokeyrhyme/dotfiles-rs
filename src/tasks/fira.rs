use crate::lib::{
    ghrafont::GhraFont,
    task::{self, Task},
};

pub fn task() -> Task {
    Task {
        name: String::from("fira"),
        sync,
        update,
    }
}

const GHRA_FONT: GhraFont = GhraFont {
    asset_re: r"^no assets, will fallback to source zip$",
    font_suffix: ".otf",
    repo: ("mozilla", "Fira"),
};

fn sync() -> task::Result {
    GHRA_FONT.sync()
}

fn update() -> task::Result {
    GHRA_FONT.update()
}
