pub trait Remember {
    fn remember(&mut self);
}


#[derive(Debug, Clone)]
pub struct MemoryCell<'l> {
    pub before: Option<&'l str>,
    pub current: Option<&'l str>,
    pub after: Option<&'l str>,
}

impl<'l> Default for MemoryCell<'l> {
    fn default() -> Self {
        MemoryCell { before: None, current: None, after: None }
    }
}

impl<'l> Remember for MemoryCell<'l> {
    fn remember(&mut self) {
        todo!();
    } 
}

#[cfg(test)]
mod test {
    use super::MemoryCell;

    #[test]
    fn test_tests() {
        assert!(true);
    }
    #[test]
    fn make_memory_cell() {
        let memory: MemoryCell;
    }
}