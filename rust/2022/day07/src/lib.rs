use std::cell::RefCell;
use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cmp;

pub mod input;

/**
    I decided to implement this exercise using polimorphism with enums. It is a bit tedious to
    pattern match a directory out of a DiskItem every single time I need to use it, but this
    pattern allows for a seamless use of size() and storage of files and directories in
    Directory.children. There are other ways of implementing this.
 */

type DiskItemType = Rc<RefCell<DiskItem>>;
type WeakDiskItemType = Weak<RefCell<DiskItem>>;

/// Represents a directory in a DiskItem
#[derive(Debug)]
pub struct Directory {
    parent: Option<WeakDiskItemType>,
    children: HashMap<String, DiskItemType>,
}

impl Directory {
    fn new(parent: Option<WeakDiskItemType>) -> Self {
        Self { parent, children: HashMap::new() }
    }

    fn add_child(&mut self, name: String, item: DiskItem) {
        self.children.insert(name, Rc::new(RefCell::new(item)));
    }

    fn get_child(&self, name: impl AsRef<str>) -> Option<&DiskItemType> {
        self.children.get(name.as_ref())
    }

    fn get_parent(&self) -> Option<DiskItemType> {
        self.parent.as_ref().map(|p| p.upgrade()
            .expect("Expect parent to exist since child hasn't been dropped yet"))
    }

    fn directories(&self) -> DirectoryIterator {
        DirectoryIterator { disk_items: self.children.values() }
    }
}

/**
    I had to implement PartialEq instead of deriving it because it isn't possible to compare
    weak references. Hence, I'm only comparing children when doing a directory comparison.
*/
impl PartialEq for Directory {
    fn eq(&self, other: &Directory) -> bool {
        self.children == other.children
    }
}

/**
    This iterator returns only directories out of a children attribute. It is backed by the
    std::collections::hash_map::Values iterator and skips over any values that aren't Directory.
    This is supposed to be used by Directory::directories(). I created it instead of just filtering
    over values to provide a nicer api.
*/
pub struct DirectoryIterator<'a> {
    disk_items: Values<'a, String, DiskItemType>
}

impl <'a> Iterator for DirectoryIterator<'a> {
    type Item = &'a DiskItemType;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.disk_items.next() {
                None => return None,
                Some(disk_item) =>
                    if let DiskItem::Directory(_) = &*disk_item.borrow() {
                        return Some(disk_item)
                    }
            }
        }
    }
}

/// Represents a file in a DiskItem
#[derive(PartialEq, Debug)]
pub struct File {
    size: usize,
}

impl File {
    fn new(size: usize) -> Self {
        Self { size }
    }
}

trait SizableDiskItem {
    fn size(&self) -> usize;
}

/// Gets the size of the current directory recursively
impl SizableDiskItem for Directory {
    fn size(&self) -> usize {
        self.children
            .values()
            .map(|item| item.borrow().size())
            .sum()
    }
}

impl SizableDiskItem for File {
    fn size(&self) -> usize {
        self.size
    }
}

/**
    This enum encapsulates disk items and makes it possible to treat them as a single polimorphic
    item.
*/
#[derive(PartialEq, Debug)]
pub enum DiskItem {
    Directory(Directory),
    File(File),
}

/**
    Helper implementation so that I don't need to pattern match both items whenever I want to get
    the size of an item.
*/
impl SizableDiskItem for DiskItem {
    fn size(&self) -> usize {
        match self {
            DiskItem::Directory(directory) => directory.size(),
            DiskItem::File(file) => file.size()
        }
    }
}

/**
    Module in charge of parsing commands in text form to a DiskItemType. In theory we could have
    other modules that derive DiskItemType from other sources.
*/
pub mod command_text_parser {
    use super::*;

    pub fn parse(lines: impl Iterator<Item=String>) -> DiskItemType {
        let root_directory = Rc::new(RefCell::new(DiskItem::Directory(Directory::new(None))));
        let mut current_directory: Option<DiskItemType> = Some(Rc::clone(&root_directory));

        for line in lines {
            if line.starts_with("$ cd") {
                let directory_name = line.split(' ').nth(2).expect("There should be a directory name after cd");
                current_directory = match directory_name {
                    "/" => get_root_directory(&root_directory),
                    ".." => get_parent_directory(current_directory),
                    directory_name => get_directory_by_name(current_directory, directory_name)
                };
            } else if !line.starts_with("$ ls") {
                let mut listing = line.split(' ');
                match listing.next().expect("First position of listing should either be dir or file size") {
                    "dir" => add_directory(&current_directory, listing
                        .next()
                        .expect("Second position of listing dir should be directory name")),
                    file_size => add_file(&current_directory, listing
                        .next()
                        .expect("Second position of listing file should be directory name"),
                        file_size),
                };
            }
        }
        root_directory
    }

    fn get_root_directory(root_directory: &DiskItemType) -> Option<DiskItemType> {
        Some(Rc::clone(root_directory))
    }

    fn get_parent_directory(current_directory: Option<DiskItemType>) -> Option<DiskItemType> {
        if let DiskItem::Directory(curr) = &*current_directory.unwrap().borrow() {
            Some(Rc::clone(&curr.get_parent()
                .expect("Should only fail if this is root, which shouldn't happen")))
        } else {
            None
        }
    }

    fn get_directory_by_name(current_directory: Option<DiskItemType>, directory_name: &str) -> Option<DiskItemType> {
        if let DiskItem::Directory(curr) = &*current_directory.unwrap().borrow() {
            Some(Rc::clone(curr.get_child(directory_name)
                .expect("Should only fail if directory doesn't exist, which shouldn't happen")))
        } else {
            None
        }
    }

    fn add_directory(current_directory: &Option<DiskItemType>, directory_name: &str) {
        if let DiskItem::Directory(directory) = &mut *current_directory
            .as_ref()
            .unwrap()
            .borrow_mut() {
                directory.add_child(directory_name.to_string(),
                                    DiskItem::Directory(
                                        Directory::new(
                                            Some(Rc::downgrade(current_directory.as_ref().unwrap())))))
        }
    }

    fn add_file(current_directory: &Option<DiskItemType>, file_name: &str, file_size: &str) {
        if let DiskItem::Directory(directory) = &mut *current_directory
            .as_ref()
            .unwrap()
            .borrow_mut() {
            directory.add_child(file_name.to_string(),
                                DiskItem::File(File::new(file_size.parse::<usize>()
                                        .expect("Expect file size to be usize"))))
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_parse_commands() {
            let root_directory = Rc::new(RefCell::new(DiskItem::Directory(Directory::new(None))));
            if let DiskItem::Directory(rtdir) = &mut *root_directory.borrow_mut() {
                rtdir.add_child("a".to_string(), DiskItem::Directory(Directory::new(Some(Rc::downgrade(&root_directory)))));
                rtdir.add_child("b.txt".to_string(), DiskItem::File(File::new(14_848_514)));
                rtdir.add_child("c.dat".to_string(), DiskItem::File(File::new(8_504_156)));
                rtdir.add_child("d".to_string(), DiskItem::Directory(Directory::new(Some(Rc::downgrade(&root_directory)))));
                let dir_a = rtdir.get_child("a").unwrap();
                if let DiskItem::Directory(a) = &mut *dir_a.borrow_mut() {
                    a.add_child("e".to_string(), DiskItem::Directory(Directory::new(Some(Rc::downgrade(dir_a)))));
                    a.add_child("f".to_string(), DiskItem::File(File::new(29_116)));
                    a.add_child("g".to_string(), DiskItem::File(File::new(2_557)));
                    a.add_child("h.lst".to_string(), DiskItem::File(File::new(62_596)));
                }
            };


            assert_eq!(parse("\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst".lines().map(String::from)), root_directory);
        }
    }
}

/// Response to the first part
pub fn sum_directory_sizes_of_100_000(lines: impl Iterator<Item=String>) -> usize {
    sum_directory_sizes_of(100_000, &command_text_parser::parse(lines))
}

fn sum_directory_sizes_of(max_size: usize, directory: &DiskItemType) -> usize {
    if let DiskItem::Directory(current_directory) = &*directory.borrow() {
        let current_directory_size = current_directory.size();
        let current_directory_size = if current_directory_size <= max_size { current_directory_size } else { 0 };

        current_directory
            .directories()
            .map(|directory| sum_directory_sizes_of(max_size, directory))
            .sum::<usize>() + current_directory_size
    } else {
        0
    }
}

/// Response to the second part
/// Returns directory size to free necessary space or -1 in case space is already free
pub fn directory_size_to_free_30_000_000(lines: impl Iterator<Item=String>) -> isize {
    let directory = command_text_parser::parse(lines);
    let min_size = 30_000_000 - (70_000_000 - directory.borrow().size() as isize);
    if min_size >= 0 {
        directory_size_to_free(min_size as usize, &directory) as isize
    } else {
        -1
    }
}

fn directory_size_to_free(min_size: usize, directory: &DiskItemType) -> usize {
    if let DiskItem::Directory(current_directory) = &*directory.borrow() {
        let current_directory_size = current_directory.size();

        let min = current_directory
            .directories()
            .map(|directory| directory_size_to_free(min_size, directory))
            .filter(|size| *size > 0)
            .min()
            .unwrap_or(0);

        if current_directory_size >= min_size && min > 0 {
            cmp::min(current_directory_size, min)
        } else if current_directory_size >= min_size {
            current_directory_size
        } else {
            0
        }
    } else {
        0
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_disk_item_directory_size() {
        let mut root_directory = Directory::new(None);
        let mut sub_directory = Directory::new(None);
        sub_directory.add_child("a".to_string(), DiskItem::File(File::new(123)));
        sub_directory.add_child("b".to_string(), DiskItem::File(File::new(7)));
        root_directory.add_child("c".to_string(), DiskItem::Directory(sub_directory));
        root_directory.add_child("d".to_string(), DiskItem::File(File::new(70)));
        assert_eq!(DiskItem::Directory(root_directory).size(), 200);
    }

    #[test]
    fn test_directories() {
        let mut root_directory = Directory::new(None);
        root_directory.add_child("a".to_string(), DiskItem::Directory(Directory::new(None)));
        root_directory.add_child("b.txt".to_string(), DiskItem::File(File::new(14_848_514)));
        root_directory.add_child("c.dat".to_string(), DiskItem::File(File::new(8_504_156)));
        root_directory.add_child("d".to_string(), DiskItem::Directory(Directory::new(None)));

        assert_eq!(root_directory.directories().count(), 2);
    }

    #[test]
    fn test_sum_directory_sizes_of() {
        assert_eq!(sum_directory_sizes_of_100_000("\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k".lines().map(String::from)), 95437);
    }

    #[test]
    fn test_directory_size_to_free_30_000_000() {
        assert_eq!(directory_size_to_free_30_000_000("\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k".lines().map(String::from)), 24933642);
    }
}