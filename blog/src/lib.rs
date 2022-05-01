use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, near_bindgen, AccountId, Balance, PanicOnDefault, Promise};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Post {
    pub owner_id: AccountId,
    pub title: String,
    pub text: String,
    pub donation: Balance,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    posts: UnorderedMap<String, Post>,
    slugs: UnorderedSet<String>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
            posts: UnorderedMap::new(b"p"),
            slugs: UnorderedSet::new(b"s"),
        }
    }

    #[payable]
    pub fn publish_post(&mut self, slug: String, title: String, text: String) {
        let account_id = env::signer_account_id();
        let donation = env::attached_deposit();
        let post = Post {
            owner_id: account_id,
            title,
            text,
            donation,
        };

        self.posts.insert(&slug, &post);
        self.slugs.insert(&slug);
    }

    pub fn get_post(&self, slug: String) -> Option<Post> {
        log!(slug);
        self.posts.get(&slug)
    }

    pub fn update_post(&mut self, slug: String, title: String, text: String) {
        self.assert_post_exists(&slug);

        let old_post = self.posts.get(&slug).unwrap();
        self.assert_post_owner(&old_post);

        let new_post = Post {
            owner_id: old_post.owner_id,
            donation: old_post.donation,
            title,
            text,
        };
        self.posts.insert(&slug, &new_post);
    }

    pub fn delete_post(&mut self, slug: String) {
        self.assert_post_exists(&slug);

        let post = self.posts.get(&slug).unwrap();
        self.assert_post_owner(&post);

        self.posts.remove(&slug);
        self.slugs.remove(&slug);
    }

    pub fn withdraw_post_donation(&mut self, slug: String) {
        self.assert_post_exists(&slug);

        let old_post = self.posts.get(&slug).unwrap();
        self.assert_post_owner(&old_post);

        let new_post = Post {
            owner_id: old_post.owner_id,
            title: old_post.title,
            text: old_post.text,
            donation: 0,
        };
        self.posts.insert(&slug, &new_post);

        let transfer_amount = old_post.donation;
        Promise::new(env::predecessor_account_id()).transfer(transfer_amount);
        log!(
            "Transfer {} to {}",
            transfer_amount,
            env::predecessor_account_id()
        );
    }

    #[payable]
    pub fn clap_post(&mut self, slug: String) {
        self.assert_post_exists(&slug);

        let old_post = self.posts.get(&slug).unwrap();
        let new_post = Post {
            owner_id: old_post.owner_id,
            title: old_post.title,
            text: old_post.text,
            donation: old_post.donation + env::attached_deposit(),
        };
        self.posts.insert(&slug, &new_post);

        log!(
            "Clap amount {} from {}",
            env::attached_deposit(),
            env::signer_account_id()
        );
    }

    pub fn list_slugs(&self) -> Vec<String> {
        self.slugs.to_vec()
    }

    fn assert_post_exists(&self, slug: &String) {
        let post = self.posts.get(&slug);
        assert!(post.is_some(), "Post not exists");
    }

    fn assert_post_owner(&self, post: &Post) {
        assert_eq!(
            env::predecessor_account_id(),
            post.owner_id,
            "Only the owner may call this method"
        );
    }
}
