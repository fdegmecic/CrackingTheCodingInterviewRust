// 3.6 - Animal Shelter
#![allow(dead_code)]

use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
enum AnimalSpecies {
    Dog,
    Cat,
}

#[derive(Debug)]
struct Animal {
    species: AnimalSpecies,
    name: String,
}

impl Animal {
    fn new_dog(name: &str) -> Self {
        Animal {
            species: AnimalSpecies::Dog,
            name: name.into(),
        }
    }

    fn new_cat(name: &str) -> Self {
        Animal {
            species: AnimalSpecies::Cat,
            name: name.into(),
        }
    }
}

#[derive(Debug)]
struct AnimalShelter {
    shelter: VecDeque<Animal>,
}

impl AnimalShelter {
    fn new() -> Self {
        AnimalShelter {
            shelter: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, animal: Animal) {
        self.shelter.push_front(animal)
    }

    fn dequeue_any(&mut self) -> Option<Animal> {
        self.shelter.pop_back()
    }

    fn dequeue_dog(&mut self) -> Option<Animal> {
        self.dequeue_oldest(AnimalSpecies::Dog)
    }

    fn dequeue_cat(&mut self) -> Option<Animal> {
        self.dequeue_oldest(AnimalSpecies::Cat)
    }

    fn dequeue_oldest(&mut self, species: AnimalSpecies) -> Option<Animal> {
        if let Some(index) = self.find_oldest_index(species) {
            return self.adopt(index);
        }
        None
    }

    fn find_oldest_index(&self, species: AnimalSpecies) -> Option<usize> {
        self.shelter
            .iter()
            .rposition(|animal| animal.species == species)
    }

    fn adopt(&mut self, index: usize) -> Option<Animal> {
        self.shelter.remove(index)
    }
}

#[cfg(test)]
mod tests {
    use super::{Animal, AnimalShelter};

    #[test]
    fn should_shelter() {
        let mut shelter = AnimalShelter::new();

        shelter.enqueue(Animal::new_dog("Dona"));
        shelter.enqueue(Animal::new_cat("Sivko"));
        shelter.enqueue(Animal::new_dog("Lily"));
        shelter.enqueue(Animal::new_cat("Nekomamushi"));
        shelter.enqueue(Animal::new_dog("Buddy"));

        assert_eq!("Dona", shelter.dequeue_any().unwrap().name);
        assert_eq!("Sivko", shelter.dequeue_cat().unwrap().name);
        assert_eq!("Lily", shelter.dequeue_dog().unwrap().name);
        assert_eq!("Nekomamushi", shelter.dequeue_any().unwrap().name);
    }
}
