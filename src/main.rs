#[derive(Debug)]
pub enum LinkedList<T> {
    Node(T, Box<LinkedList<T>>),
    End,
}

impl<T> LinkedList<T> {
	pub fn new(item: T) -> LinkedList<T> {
		LinkedList::Node(item, Box::new(LinkedList::End))
	}

	fn set_next(&mut self, new_target: LinkedList<T>) -> () {
		match self {
			LinkedList::Node(_, ref mut next) => {*next = Box::new(new_target)},
			LinkedList::End => panic!("Attempt to concat to the end of a list!"),
		};
	}

	pub fn get(&self, index: &u32) -> Result<&T, String> {
		match self {
			LinkedList::End => Err(format!("Index {} out of range of this list", index)),
			LinkedList::Node(value, next) => {
				if index > &0 {
					next.get(&(index - 1))
				} else {
					Ok(value)
				}
			}
		}
	}

	pub fn append(&mut self, second_list: LinkedList<T>) -> () {
		if let LinkedList::Node(_, next) = self {
			match **next {
				LinkedList::End => self.set_next(second_list),
				LinkedList::Node(_, _) => next.append(second_list)
			};
		} else {
			*self = second_list;
		}
	}

	pub fn split(mut self, index: &u32) -> Result<(LinkedList<T>, LinkedList<T>), String> {
		let mut current_node = &mut self;
		let mut distance_to_index = *index;
		while let LinkedList::Node(_, ref mut next) = current_node {
			if distance_to_index == 0 {
				return Ok((LinkedList::End, self))
			}
			if distance_to_index == 1 {
				let mut second_part = Box::new(LinkedList::End);
				std::mem::swap(&mut second_part, &mut *next);
				return Ok((self, *second_part))
			}
			distance_to_index -= 1;
			current_node = &mut *next;
		}
		// If we got here it means we reached an End before the index was 1 or 0
		Err(format!("Index {} out of range of this list", index))
	}

	pub fn insert(&mut self, value: T, index: &u32) -> () {
		if let LinkedList::Node(_, ref mut next) = self {
			if index == &0 {
				let mut inserted_node = LinkedList::new(value);
				std::mem::swap(self, &mut inserted_node);
				self.set_next(inserted_node);
			} else {
				next.insert(value, &(index - 1));
			};
		} else {
			panic!("Attempt to insert an item beyond the list's index range");
		}
	}

	pub fn remove(&mut self, index: &u32) -> () {
		if let LinkedList::Node(self_value, self_next) = self {
			if index == &0 {
				match **self_next {
					LinkedList::End => *self = LinkedList::End,
					LinkedList::Node(ref mut value, _) => {
						std::mem::swap(self_value, value);
						self.remove(&1);
					}
				}
			} else {
				self_next.remove(&(index - 1));
			}
		} else {
			panic!("Attempt to remove an item beyond the list's index range");
		}
	}
}

impl<T> std::ops::Index<&u32> for LinkedList<T> {
	type Output = T;

	fn index(&self, index: &u32) -> &T {
		let result = self.get(index);
		match result {
			Ok(value) => value,
			Err(message) => panic!(message)
		}
	}
}

impl<T: std::cmp::Ord> LinkedList<T> {

	pub fn sort(&mut self) -> () {
		if let LinkedList::Node(ref mut self_value, self_next) = self {
			let mut current_node = &mut **self_next;
			while let LinkedList::Node(ref mut next_value, next_next) = current_node {
				if self_value > next_value {
					std::mem::swap(self_value, next_value);
				}
				current_node = &mut **next_next;
			}
			self_next.sort();
		}
	}
}

fn main() {
    let mut x = LinkedList::new(5);
	let y = LinkedList::new(7);
	let z = LinkedList::new(9);
	x.append(y);
	x.append(z);
	println!("Appends: {:?}", x);
	x.insert(11, &0);
	println!("After insert: {:?}", x);
	x.insert(12, &2);
	println!("Another insert: {:?}", x);
	match x.split(&2) {
		Ok((part1, part2)) => {
			println!("part1: {:?}", part1);
			println!("part2: {:?}", part2);
			let mut x = part1;
			x.append(part2);
			println!("append: {:?}", x);
			x.sort();
			println!("After sort: {:?}", x);
			x.remove(&1);
			println!("After removal: {:?}", x);
			x.remove(&0);
			println!("After removal: {:?}", x);
			let slot0 = x.get(&0);
			let slot1 = x.get(&1);
			let slot2 = x.get(&2);
			println!("{:?}, {:?}, {:?}", slot0, slot1, slot2);
			println!("{:?}, {:?}, {:?}", x[&0], x[&1], x[&2]);
		},
		Err(msg) => println!("{}", msg),
	}

	//println!("{:?}, {:?}, {:?}", slot0, slot1, slot2);
	//println!("{:?}, {:?}, {:?}", x[&0], x[&1], x[&2]);
}
