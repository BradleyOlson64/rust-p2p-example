use libp2p::{ identity, PeerId };
use libp2p::gossipsub::{ self, Topic, IdentTopic};
use once_cell::sync::Lazy;
use std::hash::{DefaultHasher, Hash, Hasher};

fn main() {

}

const STORAGE_FILE_PATH: &str = "./recipes.json";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

static KEYS: Lazy<identity::Keypair> = Lazy::new(|| identity::Keypair::generate_ed25519());
static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));
static TOPIC: Lazy<IdentTopic> = Lazy::new(|| IdentTopic::new("recipes"));

type Recipes = Vec<Recipe>;

#[derive(Debug, Serialize, Deserialize)]
struct Recipe {
    id: usize,
    name: String,
    ingredients: String,
    instructions: String,
    public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
enum ListMode {
    All,
    One(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct ListRequest {
    mode: ListMode,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListResponse {
    mode: ListMode,
    data: Recipes,
    receiver: String,
}

enum EventType {
    Response(ListResponse),
    Input(String),
}