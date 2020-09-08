extern crate sdl2;

use self::sdl2::mixer;

use std::collections::HashMap;
use std::collections::VecDeque;

pub struct Player<'a> {
  music_catalog: HashMap<String, sdl2::mixer::Music<'a>>,
  current_music: Option<String>,
  playlist: Playlist,
  catalog: HashMap<String, mixer::Chunk>,
}

impl<'a> Player<'a> {
  pub fn new(pack: String) -> Player<'a> {
    let mut catalog: HashMap<String, mixer::Chunk> = HashMap::new();
    let mut music_catalog: HashMap<String, mixer::Music> = HashMap::new();
    // We should make loading dynamic.
    let pack_path = format!("assets/{}", pack);

    // Load sound effects.
    let audio_pack = [
      "ball_collision",
      "ball/collision.ogg",
      "score",
      "score/score.ogg",
    ];

    let mut i = 0;
    while i < audio_pack.len() {
      let chunk = match Player::load_audio_chunk(pack_path.clone(), audio_pack[i + 1].to_string()) {
        Ok(c) => c,
        Err(e) => panic!("{:?}", e),
      };
      catalog.insert(audio_pack[i].to_string(), chunk);
      i += 2;
    }

    // Load music.
    let music_pack = ["orchestra", "background/orchestra.ogg"];

    let mut i = 0;
    while i < music_pack.len() {
      let music_path = format!("{}/audio/{}", pack_path, music_pack[i + 1]);

      let m = match self::sdl2::mixer::Music::from_file(music_path) {
        Ok(c) => c,
        Err(e) => panic!("{:?}", e),
      };
      music_catalog.insert(music_pack[i].to_string(), m);
      i += 2;
    }

    println!("{:?}", music_catalog);

    return Player {
      playlist: Playlist::new(),
      catalog: catalog,
      music_catalog: music_catalog,
      current_music: None,
    };
  }

  pub fn load_audio_chunk(pack_path: String, asset: String) -> Result<mixer::Chunk, String> {
    let path = format!("{}/audio/{}", pack_path, asset);
    let chunk_path = std::path::Path::new(path.as_str());
    mixer::Chunk::from_file(chunk_path)
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

  // Play music item.
  pub fn play_music(&mut self, item: String, paused: bool) -> Result<(), String> {
    let m = self.music_catalog.get(&item).unwrap();
    match &self.current_music {
      Some(cm) => {
        // If we already have music playing, o\nly play the requested music if it's not already active.
        if *cm != *item {
          m.play(-1)?;
        }

        // Resume playing if the game is not paused and the requested music was already active.
        if !paused && *cm == *item {
          sdl2::mixer::Music::resume();
        }
      }
      None => {
        // If no music is active, make the requested music active and play it.
        self.current_music = Some(item);
        m.play(-1)?;
      }
    }

    Ok(())
  }

  // Pause music.
  pub fn pause_music(&mut self) {
    sdl2::mixer::Music::pause();
  }

  // Rewind music.
  pub fn rewind_music(&mut self) {
    sdl2::mixer::Music::rewind();
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
