use std::io;

use crate::lib::task::{self, Status};

pub trait Favourites {
    // found() must return the list of currently-installed favourites
    fn found(&self) -> Vec<String>;
    fn wanted(&self) -> Vec<String>;
    fn unwanted(&self) -> Vec<String>;

    // cull() must remove favourites from surplus()
    fn cull(&mut self) -> io::Result<()>;
    // fill() must install favourites from missing()
    fn fill(&mut self) -> io::Result<()>;

    // cull_and_status()
    fn cull_and_status(&mut self) -> task::Result {
        let before = self.found();
        self.cull()?;
        let after = self.found();
        if before == after {
            Ok(Status::NoChange(after.join(",")))
        } else {
            Ok(Status::Changed(before.join(","), after.join(",")))
        }
    }
    // fill_and_status()
    fn fill_and_status(&mut self) -> task::Result {
        let before = self.found();
        self.fill()?;
        let after = self.found();
        if before == after {
            Ok(Status::NoChange(after.join(",")))
        } else {
            Ok(Status::Changed(before.join(","), after.join(",")))
        }
    }

    // missing() returns the difference between wanted() and found()
    fn missing(&self) -> Vec<String> {
        let found = self.found();
        self.wanted()
            .into_iter()
            .filter_map(|f| if found.contains(&f) { None } else { Some(f) })
            .collect()
    }

    // surplus() returns the intersection of unwanted() and found()
    fn surplus(&self) -> Vec<String> {
        let found = self.found();
        self.unwanted()
            .into_iter()
            .filter_map(|f| if found.contains(&f) { Some(f) } else { None })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MyFavourites {
        install: Vec<String>,
        installed: Vec<String>,
        uninstall: Vec<String>,
    }
    impl Favourites for MyFavourites {
        fn cull(&mut self) -> io::Result<()> {
            for u in self.unwanted() {
                match self.installed.iter().position(|i| i == &u) {
                    Some(idx) => {
                        self.installed.swap_remove(idx);
                    }
                    None => {}
                }
            }
            Ok(())
        }
        fn fill(&mut self) -> io::Result<()> {
            for w in self.wanted() {
                if !self.installed.contains(&w) {
                    self.installed.push(w);
                }
            }
            Ok(())
        }

        fn found(&self) -> Vec<String> {
            self.installed.clone()
        }
        fn wanted(&self) -> Vec<String> {
            self.install.clone()
        }
        fn unwanted(&self) -> Vec<String> {
            self.uninstall.clone()
        }
    }
    impl Default for MyFavourites {
        fn default() -> MyFavourites {
            MyFavourites {
                install: vec![String::from("wanted-found"), String::from("wanted-missing")],
                installed: vec![
                    String::from("sideloaded"),
                    String::from("wanted-found"),
                    String::from("unwanted"),
                ],
                uninstall: vec![String::from("unwanted")],
            }
        }
    }

    #[test]
    fn cull_and_status_works() {
        let mut favs: MyFavourites = Default::default();
        match favs.cull_and_status() {
            Ok(s) => {
                assert_eq!(
                    Status::Changed(
                        String::from("sideloaded,wanted-found,unwanted"),
                        String::from("sideloaded,wanted-found")
                    ),
                    s
                );
            }
            Err(_) => assert!(false),
        };
        match favs.cull_and_status() {
            Ok(s) => {
                assert_eq!(Status::NoChange(String::from("sideloaded,wanted-found")), s);
            }
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn fill_and_status_works() {
        let mut favs: MyFavourites = Default::default();
        match favs.fill_and_status() {
            Ok(s) => {
                assert_eq!(
                    Status::Changed(
                        String::from("sideloaded,wanted-found,unwanted"),
                        String::from("sideloaded,wanted-found,unwanted,wanted-missing")
                    ),
                    s
                );
            }
            Err(_) => assert!(false),
        };
        match favs.fill_and_status() {
            Ok(s) => {
                assert_eq!(
                    Status::NoChange(String::from(
                        "sideloaded,wanted-found,unwanted,wanted-missing"
                    )),
                    s
                );
            }
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn missing_filters_correctly() {
        let favs: MyFavourites = Default::default();
        assert_eq!(vec![String::from("wanted-missing")], favs.missing());
    }

    #[test]
    fn surplus_filters_correctly() {
        let favs: MyFavourites = Default::default();
        assert_eq!(vec![String::from("unwanted")], favs.surplus());
    }
}
