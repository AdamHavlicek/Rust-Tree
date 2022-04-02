use std::{
    collections::HashMap,
    path::Path,
    env
};

#[derive(Debug)]
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

impl Default for Root {
    fn default() -> Root {
        Root {
            children: HashMap::new()
        }
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
                if !self.children.contains_key(&char_) {
                    self.children.insert(char_, Node::new(char_));
                } 
                child = self.children.get_mut(&char_);
            } else {
                child = child.unwrap().add_child(char_);
            }
        }
        child.unwrap().add_child('\0');
    }

    pub fn exists(&self, value: &String) -> bool {
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

fn load_words(root: &mut Root, path: impl AsRef<Path>) {
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
    };
    
    let file = File::open(path).expect("No such file");
    let buffer = BufReader::new(file);
    let lines: Vec<String> = buffer.lines()
        .map(
            |line| line.expect("Could not parse line")
        )
        .collect();

    println!("Loading words...");
    for line in lines {
        root.add_word(&line);
    }
    
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &Path = Path::new(&args[1]);
    let word = &args[2];
    let mut root = Root::default();

    load_words(&mut root, &path);

    println!("Word[{}] - {}", word, root.exists(&word));
}

#[cfg(test)]
mod node_tests {
    use crate::Node;

    #[test]
    fn it_should_create_node() {
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
}

#[cfg(test)]
mod root_tests {
    use crate::Root;

    #[test]
    fn it_should_exist() {
        // Arrange
        let word = String::from("test");
        let tree = Root::new(&word);

        // Act
        let result: bool = tree.exists(&word);

        // Assert
        assert_eq!(result, true);
    }

    #[test]
    fn it_should_not_exist() {
        // Arrange
        let word = String::from("test");
        let another_word = String::from("tes");
        let tree = Root::new(&word);

        // Act
        let result: bool = tree.exists(&another_word);

        // Assert
        assert_eq!(result, false);

    }

    #[test]
    fn it_should_return_false_with_longer_word() {
        // Arrange
        let word = String::from("test");
        let another_word = String::from("tester");
        let tree = Root::new(&word);

        // Act
        let result: bool = tree.exists(&another_word);

        // Assert
        assert_eq!(result, false);

    }
}
