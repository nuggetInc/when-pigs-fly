use std::{
    cell::RefCell,
    collections::HashSet,
    io::{self, BufRead, StdinLock},
    time::Instant,
};

/// A relation between an object with traits and abilities and another object with traits and abilities
#[derive(Debug, Clone)]
struct Relation {
    from: HashSet<String>,
    to: RefCell<HashSet<String>>,
}

impl Relation {
    /// Create a new relation from it's raw fields
    fn new(from: HashSet<String>, to: HashSet<String>) -> Self {
        Self {
            from,
            to: RefCell::new(to),
        }
    }

    /// Check if `self.to` and `other.from` match
    fn cascades(&self, other: &Self) -> bool {
        let matching = self.to.borrow().intersection(&other.from).count();

        other.from.len() == matching
    }

    /// Check if `self.from` and `other.from` match
    fn matches(&self, other: &Self) -> bool {
        self.from.intersection(&other.from).count() == self.from.len()
    }

    /// Add all items in `other.to` to `self.to`
    ///
    /// Returns `true` if `self.to` actually changed
    fn extend(&self, other: &Self) -> bool {
        let length = self.to.borrow().len();

        self.to.borrow_mut().extend(other.to.borrow().clone());

        length < self.to.borrow().len()
    }

    /// Check if this relation concludes pigs can fly
    ///
    /// `all` specifies if all pigs should be able to fly, or just some
    fn can_fly(&self, all: bool) -> bool {
        (self.from.contains("PIGS") && self.to.borrow().contains("FLY"))
            || (!all && self.to.borrow().contains("PIGS") && self.to.borrow().contains("FLY"))
    }
}

fn main() {
    let instant = Instant::now();

    let relations = read_relations();

    dbg!(instant.elapsed());

    if can_fly(relations.clone(), true) {
        println!("All pigs can fly");
    } else if can_fly(relations, false) {
        println!("Some pigs can fly");
    } else {
        println!("No pigs can fly");
    }

    dbg!(instant.elapsed());
}

/// Check if a collection of relations allow pigs to fly
///
/// `all` specifies if all pigs should be able to fly, or just some
fn can_fly(relations: Vec<Relation>, all: bool) -> bool {
    let mut changed = true;

    for relation_a in &relations {
        for relation_b in &relations {
            if std::ptr::eq(relation_a, relation_b) {
                continue;
            }

            if relation_a.matches(relation_b) && relation_b.extend(relation_a) {
                changed = true;
            }
        }
    }

    while changed {
        changed = false;

        for relation_a in &relations {
            for relation_b in &relations {
                if std::ptr::eq(relation_a, relation_b) {
                    continue;
                }

                if relation_a.cascades(relation_b) && relation_a.extend(relation_b) {
                    changed = true;
                }
            }
        }

        for relation in &relations {
            if relation.can_fly(all) {
                return true;
            }
        }
    }

    false
}

/// Read all the relations from stdin
///
/// Should start with an integer with the amount of relations
fn read_relations() -> Vec<Relation> {
    let stdin = io::stdin();
    let mut lock = stdin.lock();

    let mut buffer = String::new();
    lock.read_line(&mut buffer).unwrap();
    let count: usize = buffer.trim().parse().unwrap();

    let mut relations = Vec::with_capacity(count);

    for _ in 0..count {
        buffer.clear();
        relations.push(read_relation(&mut lock, &mut buffer))
    }

    relations
}

/// Read a single relation from stdin
fn read_relation(lock: &mut StdinLock, buffer: &mut String) -> Relation {
    lock.read_line(buffer).unwrap();

    let mut split = buffer.split_whitespace();

    let from = parse_from(&mut split);
    let to = parse_to(split);

    Relation::new(from, to)
}

/// Parse the first part of a relation statement
fn parse_from<'a>(split: &mut impl Iterator<Item = &'a str>) -> HashSet<String> {
    let mut from = HashSet::new();

    while let Some(value) = split.next() {
        from.insert(value.to_owned());

        match split.next() {
            Some("with") | Some("and") => continue,
            Some("that") => match split.next() {
                Some("can") => continue,
                _ => unreachable!(),
            },
            Some("are") | Some("have") | Some("can") => break,
            _ => unreachable!(),
        }
    }

    from
}

/// Parse the second part of a relation statement
fn parse_to<'a>(mut split: impl Iterator<Item = &'a str>) -> HashSet<String> {
    let mut to = HashSet::new();

    while let Some(value) = split.next() {
        to.insert(value.to_owned());

        match split.next() {
            Some("with") | Some("and") => continue,
            Some("that") => match split.next() {
                Some("can") => continue,
                _ => unreachable!(),
            },
            None => break,
            _ => unreachable!(),
        }
    }

    to
}
