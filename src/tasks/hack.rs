use crate::lib::{
    ghrafont::GhraFont,
    task::{self, Status, Task},
};

pub fn task() -> Task {
    Task {
        name: String::from("hack"),
        sync,
        update,
    }
}

const GHRA_FONT: GhraFont = GhraFont {
    asset_re: r"^Hack-.*-ttf\.zip$",
    font_suffix: ".ttf",
    repo: ("source-foundry", "Hack"),
};

fn sync() -> task::Result {
    GHRA_FONT.sync()
}

fn update(sync: Status) -> task::Result {
    GHRA_FONT.update(sync)
}
