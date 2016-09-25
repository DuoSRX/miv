extern crate rustbox;
use std::collections::HashMap;
use rustbox::Key;
use state::Action;
use std::mem;

#[derive(PartialEq,Eq,Debug)]
pub enum KeyMap {
    Node(HashMap<Key, KeyMap>),
    Action(Action),
}

#[derive(PartialEq,Eq,Debug)]
pub enum KeyMatch {
    None,
    Partial,
    Action(Action),
}

impl KeyMap {
    pub fn new() -> KeyMap { KeyMap::Node(HashMap::new()) }

    pub fn bind(&mut self, keys: &[Key], action: Action) {
        let new_map = match mem::replace(self, KeyMap::new()) {
            KeyMap::Node(mut keymap) => {
                if !keys.is_empty() {
                    let (k, ks) = keys.split_first().unwrap();
                    let mut km = keymap.remove(k).unwrap_or_else(KeyMap::new);
                    km.bind(ks, action);
                    keymap.insert(*k, km);
                    KeyMap::Node(keymap)
                } else if !keymap.is_empty() {
                    KeyMap::Node(keymap)
                } else {
                    KeyMap::Action(action)
                }
            }

            KeyMap::Action(_) => {
                if keys.len() > 0 {
                    let mut km = KeyMap::new();
                    km.bind(keys, action);
                    km
                } else {
                    KeyMap::Action(action)
                }
            }
        };

        *self = new_map;
    }

    pub fn match_keys(&self, keys: &[Key]) -> KeyMatch {
        match *self {
            KeyMap::Node(ref km) => {
                match keys.len() {
                    0 => KeyMatch::Partial,
                    1 => match km.get(&keys[0]) {
                        None => KeyMatch::None,
                        Some(&KeyMap::Action(ref action)) => KeyMatch::Action(action.clone()),
                        Some(&KeyMap::Node(_)) => KeyMatch::Partial,
                    },
                    _ => match km.get(&keys[0]) {
                        None => KeyMatch::None,
                        Some(&KeyMap::Action(ref action)) => KeyMatch::Action(action.clone()),
                        Some(k @ &KeyMap::Node(_)) => k.match_keys(&keys[1..])
                    }
                }
            }
            KeyMap::Action(ref action) => KeyMatch::Action(action.clone()),
        }
    }
}

pub fn key_to_string(key: Key) -> Option<String> {
    match key {
        Key::Char(c) => Some(c.to_string()),
        Key::Ctrl(c) => Some(format!("<C-{}>", c)),
        _ => None,
    }
}

#[cfg(test)]
pub mod test {
    extern crate rustbox;
    use rustbox::Key;
    use keys::*;
    use state::Action::*;

    #[test]
    fn bind() {
        let mut km = KeyMap::new();
        assert_eq!(km.match_keys(&[Key::Char('q')]), KeyMatch::None);

        km.bind(&[Key::Char('q')], Quit);
        assert_eq!(km.match_keys(&[Key::Char('q')]), KeyMatch::Action(Quit));

        km.bind(&[Key::Char('i')], Delete);
        assert_eq!(km.match_keys(&[Key::Char('i')]), KeyMatch::Action(Delete));
        assert_eq!(km.match_keys(&[Key::Char('q')]), KeyMatch::Action(Quit));

        km.bind(&[Key::Char('d'), Key::Char('d')], Delete);
        assert_eq!(km.match_keys(&[Key::Char('d')]), KeyMatch::Partial);
        assert_eq!(km.match_keys(&[Key::Char('d'), Key::Char('d')]), KeyMatch::Action(Delete));
    }
}
