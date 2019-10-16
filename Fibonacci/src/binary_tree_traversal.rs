pub struct BinaryTree<V,N>{
	value: V,
	left: Option<N>,
	right: Option<N>,
}

impl<V,N> BinaryTree<V,N> {
	pub fn get_node(value: V, left: Option<N>, right: Option<N>) -> Self {
		BinaryTree {
			value,
			left,
			right,
		}
	}

	pub fn get_leaf(value: V) -> Self {
		BinaryTree {
			value,
			left: None,
			right: None,
		}
	}

	pub fn get_left(&self) -> &Option<N> {
		&self.left
	}

	pub fn get_right(&self) -> &Option<N> {
		&self.right
	}

	pub fn is_leaf(&self) -> bool {
		self.left.is_some() && self.right.is_some()
	}
}