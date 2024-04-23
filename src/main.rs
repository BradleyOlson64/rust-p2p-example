use libp2p::{ identity, PeerId };
use once_cell::sync::Lazy;
use std::hash::{DefaultHasher, Hash, Hasher};

fn main() {
    let mut x = 2;
    let &mut ref_x = &x;
    x = 3;
    println!("Hello, world!");
}

const STORAGE_FILE_PATH: &str = "./recipes.json";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

static KEYS: Lazy<identity::Keypair> = Lazy::new(|| identity::Keypair::generate_ed25519());
static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));
static TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("recipes"));