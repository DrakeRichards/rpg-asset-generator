//! Generate a random prompt by picking from weighted lists.

mod building;
mod character;

use crate::assets::AssetType;

pub trait RandomPrompt {
    fn generate_initial_prompt(&self) -> String;
}

impl RandomPrompt for AssetType {
    fn generate_initial_prompt(&self) -> String {
        match &self {
            AssetType::Character => character::generate_initial_prompt(),
            AssetType::Location => building::generate_initial_prompt(),
        }
    }
}
