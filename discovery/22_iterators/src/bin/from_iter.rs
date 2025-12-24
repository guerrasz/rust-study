use std::collections::HashSet;

#[derive(Debug)]
#[allow(unused)]
struct Playlist {
    songs: Vec<String>,
    users: HashSet<String>,
}

// creating a from iter impl for an iterator of (String, String)
impl FromIterator<(String, String)> for Playlist {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut songs = Vec::new();
        let mut users = HashSet::new();
        for (song, user) in iter {
            songs.push(song);
            users.insert(user);
        }
        Self { songs, users }
    }
}

fn main() {
    let list_of_songs = [
        (
            String::from("Girlfriend Boyfriend"),
            String::from("Tyler, The Creator"),
        ),
        (String::from("PRIDE."), String::from("Kendrick Lamar")),
        (
            String::from("EARFQUAKE"),
            String::from("Tyler, The Creator"),
        ),
        (
            String::from("See you again"),
            String::from("Tyler, The Creator"),
        ),
    ];

    let playlist = Playlist::from_iter(list_of_songs.clone());
    println!("{:?}", playlist);
    let another_playlist: Playlist = Playlist::from_iter(list_of_songs.clone());
    println!("{:?}", another_playlist);
}
