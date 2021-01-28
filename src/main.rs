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

	fn get(&self, index: &u32) -> Result<&T, String> {
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

fn main() {
    let mut x = LinkedList::new(5);
	let y = LinkedList::new(7);
	x.set_next(y);
	println!("{:?}", x);
	let slot0 = x.get(&0);
	let slot1 = x.get(&1);
	let slot2 = x.get(&2);
	println!("{:?}, {:?}, {:?}", slot0, slot1, slot2);
	println!("{:?}, {:?}", x[&0], x[&1]);
}
