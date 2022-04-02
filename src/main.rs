use std::collections::HashMap;

struct Node {
    children: HashMap<char, Node>,
    data: char,
}

struct Root {
    children: HashMap<char, Node>
}

impl Node {
    pub fn new(value: char) -> Self {
        Self {
            children: HashMap::new(),
            data: value,
        }
    }

    pub fn add_child(&mut self, value: char) -> Option<&mut Node> {
        if !self.children.contains_key(&value) {
            let child: Node = Node::new(value);
            self.children.insert(value, child);
        }

        return self.children.get_mut(&value)
    }

}

impl Root {
    pub fn new(value: &String) -> Self {
        let mut instance = Self {
            children: HashMap::new(),
        };
        
        instance.add_word(value);

        instance
    }

    pub fn add_word(&mut self, value: &String) {
        let chars = value.chars();

        let mut child: Option<&mut Node> = None;
        for char_ in chars {
            if child.is_none() {
                self.children.insert(char_, Node::new(char_));
                child = self.children.get_mut(&char_);
            } else {
                child = child.unwrap().add_child(char_);
            }
        }

        child.unwrap().add_child('\0');
    }

    pub fn exists(&self, value: String) -> bool {
        let mut node: Option<&Node> = None;
        for index_char in value.char_indices(){
            if node.is_none() && index_char.0 == 0 {
                node = self.children.get(&index_char.1);
            } else if node.is_none() {
                break;
            } else {
                node = node.unwrap().children.get(&index_char.1)
            }
        }

        if node.is_none() {
            return false
        } else {
            return !node.unwrap().children.get(&'\0').is_none()
        }
    }
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_should_create_tree() {
        // Arrange
        let value: char = 'a';
        let node: Node = Node::new(value);

        // Act
        let result: char = node.data;
    
        // Assert
        assert_eq!(result, value);
    }

    #[test]
    fn it_should_add_node_to_children() {
        // Arrange
        let value: char = 'p';
        let mut node: Node = Node::new('a');

        // Act
        let result = node.add_child(value).unwrap();

        // Assert
        assert_eq!(value, result.data);
        assert_eq!(0, result.children.len());
        assert_eq!(1, node.children.len());
        assert_eq!(true, node.children.contains_key(&value))
    }

    #[test]
    fn it_should_exist() {
        // Arrange
        let word = String::from("Test");
        let tree = Root::new(&word);

        // Act
        let result: bool = tree.exists(word);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn it_should_not_exist() {
        // Arrange
        let word = String::from("Test");
        let another_word = String::from("Tes");
        let tree = Root::new(&word);

        // Act
        let result: bool = tree.exists(another_word);

        // Assert
        assert_eq!(result, false);

    }
}