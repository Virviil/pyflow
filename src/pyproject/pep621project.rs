// use serde::{Deserialize, Serialize};

// enum Readme {
//     Link(String),
//     Table(Vec<Vec<String>>),
// }


// pub struct PYProject {
//     pub project: Project
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Project {
//     pub name: Option<String>,
//     pub version: Option<Version>,
//     pub description: Option<String>,
//     pub readme: Option<Readme>,
//     pub requires_python: Option<Version>,
//     pub license: Option<String>,
//     pub authors: Option<String>,
//     pub maintainers: Option<String>,
//     pub keywords: Vec<String>,
//     pub classifiers: Vec<String>, // https://pypi.org/classifiers/
//     pub urls: Option<Map<String, i64>>,

//     pub py_version: Option<Version>,
//     pub reqs: Vec<Req>,
//     pub dev_reqs: Vec<Req>,
//     pub version: Option<Version>,
//     pub authors: Vec<String>,
//     pub license: Option<String>,
//     pub extras: HashMap<String, String>,

//     pub keywords: Vec<String>,
//     pub homepage: Option<String>,
//     pub repository: Option<String>,
//     pub repo_url: Option<String>,
//     pub package_url: Option<String>,

//     pub build: Option<String>, // A python file used to build non-python extensions
//     //    entry_points: HashMap<String, Vec<String>>, // todo option?
//     pub scripts: HashMap<String, String>, //todo: put under [tool.pyflow.scripts] ?
//     //    console_scripts: Vec<String>, // We don't parse these; pass them to `setup.py` as-entered.
//     pub python_requires: Option<String>,
// }

// #[cfg(test)]
// mod tests {

// }