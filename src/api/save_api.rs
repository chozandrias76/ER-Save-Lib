use std::{
    collections::{BTreeMap, HashMap},
    num::ParseIntError,
    path::Path,
};

use crate::{
    regulation::{params::param_structs, regulation::RegulationParseError},
    save::save::SaveParseError,
    Save,
};

use super::event_flags::EventFlagsApi;

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

pub struct Param<P: param_structs::param_trait::Param> {
    pub rows: HashMap<i32, P::ParamType>,
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

    /// Converts the save data to a vector of bytes.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// let bytes = save_api.to_vec().unwrap();
    /// ```
    pub fn to_vec(&self) -> Result<Vec<u8>, SaveApiError> {
        let bytes = self.raw.write_to_vec()?;
        Ok(bytes)
    }

    /// Writes the save data to the specified path.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.write_to_path("./test/null.sl2").unwrap();
    /// ```
    pub fn write_to_path(&self, path: impl AsRef<Path>) -> Result<(), SaveApiError> {
        Ok(self.raw.write_to_path(path)?)
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

    /// Returns the platform type of the save file.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::{SaveApi, SaveType};
    /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// let platform = save_api.platform();
    /// assert_eq!(platform, SaveType::PC);
    /// ```
    pub fn platform(&self) -> SaveType {
        if self.raw.header.len() == 0x6c {
            SaveType::Playstation
        } else {
            SaveType::PC
        }
    }

    /// Returns the index of the character with the given name.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// let index = save_api.character_index_from_name("CharacterName");
    /// ```
    pub fn character_index_from_name(&self, name: &str) -> Option<usize> {
        self.raw
            .user_data_10
            .profile_summary
            .profiles
            .iter()
            .position(|profile| profile.character_name.contains(name))
    }

    /// Returns a map of parameter bytes.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// let param_bytes_map = save_api.get_param_bytes_map().unwrap();
    /// ```
    pub fn get_param_bytes_map(&self) -> Result<&BTreeMap<String, Vec<u8>>, SaveApiError> {
        Ok(&self
            .raw
            .user_data_11
            .regulation
            .content
            .data
            .file_data
            .param_files)
    }

    /// Returns parameters by type
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// use er_save_lib::TalkParam::TalkParam;
    /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// let param = save_api.get_param::<TalkParam>().unwrap();
    /// ```
    pub fn get_param<P: param_structs::param_trait::Param>(
        &self,
    ) -> Result<Param<P>, SaveApiError> {
        let rows = self.raw.user_data_11.regulation.get_param::<P>()?;
        Ok(Param::<P> { rows })
    }

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

    /// Returns the Steam ID associated with the save file.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// let steam_id = save_api.steam_id();
    /// ```
    pub fn steam_id(&self) -> u64 {
        self.raw.user_data_10.steam_id
    }

    /// Sets the Steam ID associated with the save file.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.set_steam_id(1234567890).unwrap();
    /// ```
    pub fn set_steam_id(&mut self, steam_id: u64) -> Result<(), SaveApiError> {
        self.raw.user_data_10.steam_id = steam_id;
        Ok(())
    }

    /// Returns the name of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// let name = save_api.character_name(0);
    /// ```
    pub fn character_name(&self, index: usize) -> String {
        self.raw.user_data_x[index]
            .player_game_data
            .character_name
            .to_string()
    }

    /// Sets the name of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.set_character_name(0, "NewName").unwrap();
    /// ```
    pub fn set_character_name(&mut self, index: usize, new_name: &str) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.character_name = new_name.to_string();
        self.raw.user_data_10.profile_summary.profiles[index].character_name = new_name.to_string();
        Ok(())
    }

    /// Returns an array indicating which characters are active.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// let active_characters = save_api.active_characters();
    /// ```
    pub fn active_characters(&self) -> [bool; 10] {
        self.raw.user_data_10.profile_summary.active_profiles
    }

    /// Returns the gender of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// let gender = save_api.gender(0);
    /// ```
    pub fn gender(&self, index: usize) -> u8 {
        self.raw.user_data_x[index].player_game_data.gender
    }

    /// Sets the gender of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// let (index, gender_id) = (0, 1);
    /// save_api.set_gender(index, gender_id);
    /// ```
    pub fn set_gender(&mut self, index: usize, gender: u8) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.gender = gender;
        self.raw.user_data_10.profile_summary.profiles[index].gender = gender;
        Ok(())
    }

    /// Returns the id of the archetype for the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// let archetype = save_api.archetype(0);
    /// ```
    pub fn archetype(&self, index: usize) -> u8 {
        self.raw.user_data_x[index].player_game_data.archetype
    }

    /// Sets the archetype of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// let (index, archetype_id) = (0, 1);
    /// save_api.set_archetype(index, archetype_id);
    /// ```
    pub fn set_archetype(&mut self, index: usize, archetype: u8) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.archetype = archetype;
        self.raw.user_data_10.profile_summary.profiles[index].archetype = archetype;
        Ok(())
    }

    /// Returns the hp for the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// let hp = save_api.hp(0);
    /// ```
    pub fn hp(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.hp
    }

    /// Sets the hp of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// let (index, hp) = (0, 1_000);
    /// save_api.set_hp(index, hp);
    /// ```
    pub fn set_hp(&mut self, index: usize, hp: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.hp = hp;
        Ok(())
    }

    
    /// Gets the max hp of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// let index = 0;
    /// save_api.max_hp(index);
    /// ```
    pub fn max_hp(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.max_hp
    }

    /// Sets the max hp of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// let (index, max_hp) = (0, 1_000);
    /// save_api.set_max_hp(index, max_hp);
    /// ```
    pub fn set_max_hp(&mut self, index: usize, max_hp: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.max_hp = max_hp;
        Ok(())
    }

    /// Gets the base max hp of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// let index = 0;
    /// save_api.base_max_hp(index);
    /// ```
    pub fn base_max_hp(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.base_max_hp
    }

    /// Sets the base max hp of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// let (index, base_max_hp) = (0, 1_000);
    /// save_api.set_base_max_hp(index, base_max_hp);
    /// ```
    pub fn set_base_max_hp(&mut self, index: usize, base_max_hp: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.base_max_hp = base_max_hp;
        Ok(())
    }

    /// Sets the fp of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.set_fp(0, 1);
    /// ```
    pub fn set_fp(&mut self, index: usize, fp: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.fp = fp;
        Ok(())
    }

    /// Gets the max fp of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.max_fp(0);
    /// ```
    pub fn max_fp(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.max_fp
    }

    /// Sets the max fp of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.set_max_fp(0, 1);
    /// ```
    pub fn set_max_fp(&mut self, index: usize, max_fp: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.max_fp = max_fp;
        Ok(())
    }

    /// Gets the base max fp of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.base_max_fp(0);
    /// ```
    pub fn base_max_fp(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.base_max_fp
    }

    /// Sets the base max fp of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.set_base_max_fp(0, 1);
    /// ```
    pub fn set_base_max_fp(&mut self, index: usize, base_max_fp: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.base_max_fp = base_max_fp;
        Ok(())
    }

    /// Sets the sp of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.set_sp(0, 1);
    /// ```
    pub fn set_sp(&mut self, index: usize, sp: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.sp = sp;
        Ok(())
    }

    /// Gets the max sp of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.max_sp(0);
    /// ```
    pub fn max_sp(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.max_sp
    }

    /// Sets the max sp of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.set_max_sp(0, 1);
    /// ```
    pub fn set_max_sp(&mut self, index: usize, max_sp: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.max_sp = max_sp;
        Ok(())
    }

    /// Gets the base max sp of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.base_max_sp(0);
    /// ```
    pub fn base_max_sp(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.base_max_sp
    }

    /// Sets the base max sp of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.set_base_max_sp(0, 1);
    /// ```
    pub fn set_base_max_sp(&mut self, index: usize, base_max_sp: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.base_max_sp = base_max_sp;
        Ok(())
    }

    /// Gets the level of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.level(0);
    /// ```
    pub fn level(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.level
    }

    /// Sets the level of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.set_level(0, 1);
    /// ```
    pub fn set_level(&mut self, index: usize, level: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.level = level;
        self.raw.user_data_10.profile_summary.profiles[index].level = level;
        Ok(())
    }

    /// Gets the vigor of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.vigor(0);
    /// ```
    pub fn vigor(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.vigor
    }

    /// Sets the vigor of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.set_vigor(0, 1);
    /// ```
    pub fn set_vigor(&mut self, index: usize, vigor: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.vigor = vigor;
        Ok(())
    }

    /// Gets the mind of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.mind(0);
    /// ```
    pub fn mind(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.mind
    }

    /// Sets the mind of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.set_mind(0, 1);
    /// ```
    pub fn set_mind(&mut self, index: usize, mind: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.mind = mind;
        Ok(())
    }

    /// Gets the endurance of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.endurance(0);
    /// ```
    pub fn endurance(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.endurance
    }

    /// Sets the endurance of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.set_endurance(0, 1);
    /// ```
    pub fn set_endurance(&mut self, index: usize, endurance: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.endurance = endurance;
        Ok(())
    }

    /// Gets the strength of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.strength(0);
    /// ```
    pub fn strength(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.strength
    }

    /// Sets the strength of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.set_strength(0, 1);
    /// ```
    pub fn set_strength(&mut self, index: usize, strength: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.strength = strength;
        Ok(())
    }

    /// Gets the dexterity of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.dexterity(0);
    /// ```
    pub fn dexterity(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.dexterity
    }

    /// Sets the dexterity of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.set_dexterity(0, 1);
    /// ```
    pub fn set_dexterity(&mut self, index: usize, dexterity: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.dexterity = dexterity;
        Ok(())
    }

    /// Gets the intelligence of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.intelligence(0);
    /// ```
    pub fn intelligence(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.intelligence
    }

    /// Sets the intelligence of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.set_intelligence(0, 1);
    /// ```
    pub fn set_intelligence(
        &mut self,
        index: usize,
        intelligence: u32,
    ) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.intelligence = intelligence;
        Ok(())
    }

    /// Gets the faith of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.faith(0);
    /// ```
    pub fn faith(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.faith
    }

    /// Sets the faith of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.set_faith(0, 1);
    /// ```
    pub fn set_faith(&mut self, index: usize, faith: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.faith = faith;
        Ok(())
    }

    /// Gets the arcane of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.arcane(0);
    /// ```
    pub fn arcane(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.arcane
    }

    /// Gets the arcane of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.set_arcane(0, 1);
    /// ```
    pub fn set_arcane(&mut self, index: usize, arcane: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.arcane = arcane;
        Ok(())
    }

    /// Gets the runes of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.runes(0);
    /// ```
    pub fn runes(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.runes
    }

    /// Sets the runes of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.set_runes(0, 1_000);
    /// ```
    pub fn set_runes(&mut self, index: usize, runes: u32) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.runes = runes;
        Ok(())
    }

    /// Gets the rune memory of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.runes_memory(0);
    /// ```
    pub fn runes_memory(&self, index: usize) -> u32 {
        self.raw.user_data_x[index].player_game_data.runes_memory
    }

    /// Sets the rune memory of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.set_runes_memory(0, 1_000);
    /// ```
    pub fn set_runes_memory(
        &mut self,
        index: usize,
        runes_memory: u32,
    ) -> Result<(), SaveApiError> {
        self.raw.user_data_x[index].player_game_data.runes_memory = runes_memory;
        self.raw.user_data_10.profile_summary.profiles[index].runes_memory = runes_memory;
        Ok(())
    }

    /// Gets the regions of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.regions(0);
    /// ```
    pub fn regions(&self, index: usize) -> Result<&Vec<u32>, SaveApiError> {
        Ok(&self.raw.user_data_x[index].unlocked_regions.ids)
    }

    /// Gets the regions count of the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.regions_count(0);
    /// ```
    pub fn regions_count(&self, index: usize) -> Result<u32, SaveApiError> {
        Ok(self.raw.user_data_x[index].unlocked_regions.count)
    }

    /// Adds a region to the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.add_region(0, 1_000);
    /// ```
    pub fn add_region(&mut self, index: usize, region_id: u32) -> Result<(), SaveApiError> {
        let user_data_x = &mut self.raw.user_data_x[index];
        if user_data_x
            .unlocked_regions
            .ids
            .iter()
            .position(|id| *id == region_id)
            .is_none()
        {
            user_data_x.unlocked_regions.ids.push(region_id);
            user_data_x.unlocked_regions.count += 1;
            let rest_len = user_data_x.rest.len();
            user_data_x.rest.truncate(rest_len - 4);
        }
        Ok(())
    }

    /// Removes a region from the character at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use er_save_lib::SaveApi;
    /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
    /// save_api.remove_region(0, 1_000);
    /// ```
    pub fn remove_region(&mut self, index: usize, region_id: u32) -> Result<(), SaveApiError> {
        let user_data_x = &mut self.raw.user_data_x[index];
        if let Some(region_index) = user_data_x
            .unlocked_regions
            .ids
            .iter()
            .position(|id| *id == region_id)
        {
            user_data_x.unlocked_regions.ids.remove(region_index);
            user_data_x.unlocked_regions.count -= 1;
            user_data_x.rest.extend(vec![0; 4]);
        }
        Ok(())
    }
}
