use crate::lib::{
    ghrafont::GhraFont,
    task::{self, Status, Task},
};

pub fn task() -> Task {
    Task {
        name: String::from("sourcesanspro"),
        sync,
        update,
    }
}

const GHRA_FONT: GhraFont = GhraFont {
    asset_re: r"^source-sans-pro-.*\.zip$",
    font_suffix: ".otf",
    repo: ("adobe-fonts", "source-sans-pro"),
};

fn sync() -> task::Result {
    GHRA_FONT.sync()
}

fn update(sync: Status) -> task::Result {
    GHRA_FONT.update(sync)
}
