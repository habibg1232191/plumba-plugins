#[macro_use]
extern crate dlopen_derive;

use std::{fs, io};
use std::io::Error;
use std::ops::Index;
use std::path::Path;
use std::slice::Iter;
use dlopen::wrapper::{Container, WrapperApi};
use serde::{Deserialize, Serialize};
use crate::title_plugin::PlumbaPlugin;

pub mod title_plugin;

#[derive(Default)]
pub struct PlumbaPluginSystem {
    plugins: Vec<Box<dyn PlumbaPlugin>>,
    _plugin_api: Vec<Container<PluginApi>>
}

impl Index<usize> for PlumbaPluginSystem {
    type Output = Box<dyn PlumbaPlugin>;

    fn index(&self, index: usize) -> &Self::Output {
        self.plugins.get(index).unwrap()
    }
}

impl PlumbaPluginSystem {
    pub fn load(&mut self, path: &Path) {
        let plugin_api_wrapper: Container<PluginApi> = unsafe { Container::load(path) }.unwrap();
        let plugin = unsafe { Box::from_raw(plugin_api_wrapper.create_plugin()) };
        self.plugins.push(plugin);
        self._plugin_api.push(plugin_api_wrapper);
    }

    pub fn iter(&self) -> Iter<'_, Box<dyn PlumbaPlugin>> {
        self.plugins.iter()
    }

    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    pub fn load_directory(&mut self, directory: &Path) -> io::Result<()> {
        let paths = match fs::read_dir(directory) {
            Ok(path) => path,
            Err(_) => return Err(Error::last_os_error())
        };

        for path in paths {
            if let Ok(path) = path {
                if path.file_name().to_str().unwrap().contains("_plugin") {
                    self.load(&path.path())
                }
            } else {
                return Err(Error::last_os_error())
            }
        }

        Ok(())
    }

    pub fn get_plugin(&self, id: usize) -> &Box<dyn PlumbaPlugin> {
        &self.plugins[id]
    }

    pub fn get_mut_plugin(&mut self, id: usize) -> &mut Box<dyn PlumbaPlugin> {
        self.plugins.get_mut(id).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub icon_data: &'static [u8]
}

impl PluginInfo {
    pub fn to_json(&self) -> serde_json::error::Result<String> {
        serde_json::to_string(self)
    }
}

#[warn(improper_ctypes_definitions)]
#[derive(WrapperApi)]
struct PluginApi {
    create_plugin: extern fn() -> *mut dyn PlumbaPlugin,
}

const PATH_PLUGINS: &str = "/home/lolkapm/RustroverProjects/title-plugin-system/plugins";

#[cfg(test)]
mod test {
    use std::path::Path;
    use crate::{PATH_PLUGINS, PlumbaPluginSystem};

    #[test]
    fn run_plugin() {
        let mut plumba_system = PlumbaPluginSystem::default();
        plumba_system.load_directory(&Path::new(PATH_PLUGINS)).expect("Error when loaded");
    }
}