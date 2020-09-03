extern crate sdl2;

use self::sdl2::mixer;

use std::collections::HashMap;
use std::collections::VecDeque;

pub struct Player {
  playlist: Playlist,
  catalog: HashMap<String, mixer::Chunk>,
}

impl Player {
  pub fn new(pack: String) -> Player {
    let mut catalog: HashMap<String, mixer::Chunk> = HashMap::new();

    // We should make loading dynamic.
    let pack_path = format!("assets/{}/audio/ball/collision.ogg", pack);
    let ball_collision_path = std::path::Path::new(pack_path.as_str());
    let ball_collision_chunk = match mixer::Chunk::from_file(ball_collision_path) {
      Ok(a) => a,
      Err(e) => panic!("{:?}", e),
    };

    catalog.insert("ball_collision".to_string(), ball_collision_chunk);

    return Player {
      playlist: Playlist::new(),
      catalog: catalog,
    };
  }

  pub fn add(&mut self, spec: String) {
    self.playlist.add(spec);
  }

  // Play each item in the playlist.
  pub fn play(&mut self) -> Result<(), String> {
    while let Some(item) = self.playlist.items.pop_front() {
      let chunk = self.catalog.get(&item).unwrap();

      mixer::Channel::all().play(&chunk, 0)?;
    }

    Ok(())
  }
}

// Playlist holds a list of items to be played.
struct Playlist {
  pub items: VecDeque<String>,
}

impl Playlist {
  // Return a new playlist.
  fn new() -> Playlist {
    Playlist {
      items: VecDeque::with_capacity(16),
    }
  }

  // Add an item to the playlist to be played on next update.
  fn add(&mut self, spec: String) {
    self.items.push_back(spec);
  }
}

#[test]
fn test_playlist() {
  let playlist = Playlist::new();

  assert!(
    playlist.items.len() == 0,
    "expected 0 but received: {}",
    playlist.items.len()
  );
}
