use std::collections::HashMap;
use std::hash::Hash;

use edfsm::Fsm;

#[derive(Clone)]
pub struct State<K, V>(HashMap<K, V>);

enum Command<K, V, C> {
    Insert(K, V),
    Delete(K),
    Execute(K, C),
}

#[derive(Clone)]
enum Event<K, V, E> {
    Inserted(K, V),
    Deleted(K),
    Changed(K, E),
}

trait Effects<K, V> {
    type Ve;
    fn inserting(&mut self, k: &K, v: &V);
    fn deleting(&mut self, k: &K);
    fn executing(&mut self, k: &K) -> Self::Ve;
}

struct Kvsm<M>(M);

impl<K, V, C, E, SE, M> Fsm<State<K, V>, Command<K, V, C>, Event<K, V, E>, SE> for Kvsm<M>
where
    M: Fsm<V, C, E, SE::Ve>,
    SE: Effects<K, V>,
    K: Eq + Hash + Clone,
    V: Clone,
    E: Clone,
{
    fn for_command(s: &State<K, V>, c: Command<K, V, C>, se: &mut SE) -> Option<Event<K, V, E>> {
        match c {
            Command::Insert(k, v) => {
                se.inserting(&k, &v);
                if s.0.contains_key(&k) {
                    None
                } else {
                    Some(Event::Inserted(k, v))
                }
            }
            Command::Delete(k) => {
                se.deleting(&k);
                if s.0.contains_key(&k) {
                    Some(Event::Deleted(k))
                } else {
                    None
                }
            }
            Command::Execute(k, c) => {
                s.0.get(&k)
                    .and_then(|v| M::for_command(v, c, &mut se.executing(&k)))
                    .map(|e| Event::Changed(k, e))
            }
        }
    }

    fn on_event(s: &State<K, V>, e: &Event<K, V, E>) -> Option<State<K, V>> {
        let e = e.clone();
        match e {
            Event::Inserted(k, v) => {
                let mut s1 = s.clone();
                s1.0.insert(k, v);
                Some(s1)
            }
            Event::Deleted(k) => {
                let mut s = s.clone();
                s.0.remove(&k);
                Some(s)
            }
            Event::Changed(k, e) => s.0.get(&k).and_then(|v| {
                M::on_event(v, &e).map(|v| {
                    let mut s = s.clone();
                    s.0.insert(k, v);
                    s
                })
            }),
        }
    }

    fn is_transitioning(s0: &State<K, V>, s1: &State<K, V>) -> bool {
        s0.0.len() != s1.0.len()
    }
}
