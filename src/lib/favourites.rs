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
        let diff = diff_before_after(before, after);
        if diff.0.len() == 0 && diff.1.len() == 0 {
            Ok(Status::NoChange(String::new()))
        } else {
            Ok(Status::Changed(diff.0.join(","), diff.1.join(",")))
        }
    }
    // fill_and_status()
    fn fill_and_status(&mut self) -> task::Result {
        let before = self.found();
        self.fill()?;
        let after = self.found();
        let diff = diff_before_after(before, after);
        if diff.0.len() == 0 && diff.1.len() == 0 {
            Ok(Status::NoChange(String::new()))
        } else {
            Ok(Status::Changed(diff.0.join(","), diff.1.join(",")))
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

// diff_before_after() returns favourites that are newly absent and newly present
fn diff_before_after<S>(before: Vec<S>, after: Vec<S>) -> (Vec<String>, Vec<String>)
where
    S: Into<String> + AsRef<str> + PartialEq,
{
    let absent: Vec<String> = before
        .iter()
        .filter(|b| !after.contains(&b))
        .map(|b| String::from(b.as_ref()))
        .collect();
    let present: Vec<String> = after
        .iter()
        .filter(|a| !before.contains(&a))
        .map(|a| String::from(a.as_ref()))
        .collect();
    (absent, present)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_before_after_works() {
        let before = vec!["a", "b", "c", "d"];
        let after = vec!["c", "d", "e", "f"];
        let got = diff_before_after(before, after);
        assert_eq!(vec!["a", "b"], got.0);
        assert_eq!(vec!["e", "f"], got.1);
    }

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
                    Status::Changed(String::from("unwanted"), String::from("")),
                    s
                );
            }
            Err(_) => assert!(false),
        };
        match favs.cull_and_status() {
            Ok(s) => {
                assert_eq!(Status::NoChange(String::from("")), s);
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
                    Status::Changed(String::from(""), String::from("wanted-missing")),
                    s
                );
            }
            Err(_) => assert!(false),
        };
        match favs.fill_and_status() {
            Ok(s) => {
                assert_eq!(Status::NoChange(String::from("")), s);
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
