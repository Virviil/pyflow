// use std::iter::Map;

// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// pub enum Readme {
//     Link(String),
//     DynamicLink(ReadmeDynamicLink),
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct ReadmeDynamicLink {
//     pub file: Option<String>,
//     pub text: Option<String>,
//     #[serde(rename = "content-type")]
//     pub content_type: Option<String>,
//     pub charset: Option<String>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct License {
//     pub file: Option<String>,
//     pub text: Option<String>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Author {
//     pub name: Option<String>,
//     pub email: Option<String>,
// }

// pub struct PyProject {
//     pub project: Option<Project>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Project {
//     pub name: Option<String>,
//     pub version: Option<String>,
//     pub description: Option<String>,
//     pub readme: Option<Readme>,
//     pub requires_python: Option<String>,
//     pub license: Option<License>,
//     // Authors and maintainers
//     pub authors: Option<Vec<Author>>,
//     pub maintainers: Option<Vec<Author>>,
//     pub keywords: Vec<String>,
//     pub classifiers: Vec<String>, // https://pypi.org/classifiers/
//     pub urls: Option<Map<String, String>>,
//     // Entry points
//     pub scripts: String,
//     #[serde(rename = "gui-scripts")]
//     pub gui_scripts: String,
//     #[serde(rename = "entry-points")]
//     pub entry_points: String,
// }

// #[cfg(test)]
// mod tests {
//     const PYPROJECT_TOML: &str = indoc::indoc! {r#"
//     [project]
//     name = "spam"
//     version = "2020.0.0"
//     description = "Lovely Spam! Wonderful Spam!"
//     readme = "README.rst"
//     requires-python = ">=3.8"
//     license = {file = "LICENSE.txt"}
//     keywords = ["egg", "bacon", "sausage", "tomatoes", "Lobster Thermidor"]
//     authors = [
//       {email = "hi@pradyunsg.me"},
//       {name = "Tzu-Ping Chung"}
//     ]
//     maintainers = [
//       {name = "Brett Cannon", email = "brett@python.org"}
//     ]
//     classifiers = [
//       "Development Status :: 4 - Beta",
//       "Programming Language :: Python"
//     ]
    
//     dependencies = [
//       "httpx",
//       "gidgethub[httpx]>4.0.0",
//       "django>2.1; os_name != 'nt'",
//       "django>2.0; os_name == 'nt'"
//     ]
    
//     [project.optional-dependencies]
//     test = [
//       "pytest < 5.0.0",
//       "pytest-cov[all]"
//     ]
    
//     [project.urls]
//     homepage = "example.com"
//     documentation = "readthedocs.org"
//     repository = "github.com"
//     changelog = "github.com/me/spam/blob/master/CHANGELOG.md"
    
//     [project.scripts]
//     spam-cli = "spam:main_cli"
    
//     [project.gui-scripts]
//     spam-gui = "spam:main_gui"
    
//     [project.entry-points."spam.magical"]
//     tomatoes = "spam:main_tomatoes"
//     "#};
// }
