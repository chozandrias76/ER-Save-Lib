pub mod save_data_api;
pub mod user_data_10_api;
pub mod user_data_11_api;
pub mod user_data_api;
pub mod user_data_x_api;

use std::{
    num::ParseIntError,
    path::Path,
};

use super::event_flags::EventFlagsApi;
use crate::{
    regulation::{regulation::RegulationParseError},
    save::save::SaveParseError,
    Save,
};

#[derive(thiserror::Error, Debug)]
pub enum SaveApiError {
    #[error(transparent)]
    DekuError(#[from] deku::DekuError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SaveParserError(#[from] SaveParseError),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("EventId {} not found!", .0)]
    EventIdNotFound(u32),
    #[error(transparent)]
    RegulationParseError(#[from] RegulationParseError),
}

#[derive(PartialEq, Debug)]
pub enum SaveType {
    PC,
    Playstation,
}


pub struct SaveApi {
    raw: Save,
}

impl SaveApi {
    /// Creates a new `SaveApi` instance from a `Save` object.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// use er_save_lib::Save;
    /// let save = Save::from_path("./test/PS_Save.txt").unwrap();
    /// let save_api = SaveApi::new(save);
    /// ```
    pub fn new(save: Save) -> Self {
        SaveApi { raw: save }
    }

    /// Creates a `SaveApi` instance from a slice of bytes.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let bytes = std::fs::read("./test/ER0000.sl2").unwrap();
    /// let save_api = SaveApi::from_slice(&bytes).unwrap();
    /// ```
    pub fn from_slice(bytes: &[u8]) -> Result<Self, SaveApiError> {
        let raw = Save::from_slice(bytes)?;
        Ok(SaveApi { raw })
    }

    /// Creates a `SaveApi` instance from a file path.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// ```
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, SaveApiError> {
        let raw = Save::from_path(path)?;
        Ok(SaveApi { raw })
    }
}

impl SaveApi {
    /// Returns event flags by id and character index
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// use er_save_lib::TalkParam::TalkParam;
    /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// // IDs can be found in src\res\eventflag_bst.txt as the second element of each tuple
    /// let event_flag = save_api.get_event_flag(6223, 0).unwrap();
    /// ```
    pub fn get_event_flag(
        &self,
        event_id: u32,
        character_index: usize,
    ) -> Result<bool, SaveApiError> {
        EventFlagsApi::get_event_flag(&self.raw, event_id, character_index)
    }

    /// Sets the value of the specified event flag for the given character index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.set_event_flag(123, 0, true).unwrap();
    /// ```
    pub fn set_event_flag(
        &mut self,
        event_id: u32,
        character_index: usize,
        on: bool,
    ) -> Result<(), SaveApiError> {
        EventFlagsApi::set_event_flag(&mut self.raw, event_id, character_index, on)
    }
}
