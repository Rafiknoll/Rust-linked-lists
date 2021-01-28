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
}

fn main() {
    let mut x = LinkedList::new(5);
	let y = LinkedList::new(7);
	x.set_next(y);
	println!("{:?}", x);
}
