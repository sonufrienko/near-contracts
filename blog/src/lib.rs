use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct BlogContract {
    posts: UnorderedMap<String, String>,
}

#[near_bindgen]
impl BlogContract {
    #[init]
    pub fn new() -> Self {
        Self {
            posts: UnorderedMap::new(b"s".to_vec()),
        }
    }

    pub fn publish_post(&mut self, slug: String, text: String) {
        self.posts.insert(&slug, &text);
    }

    pub fn get_post(&self, slug: String) -> Option<String> {
        self.posts.get(&slug)
    }
}
