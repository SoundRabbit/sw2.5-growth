#[derive(Clone)]
pub struct Attrs {
    pub selecting: usize,
    pub attrs_list: Vec<[i32; 6]>,
}

impl Attrs {
    pub fn new(n: usize) -> Self {
        let mut attrs_list = vec![];
        for _ in 0..usize::max(1, n) {
            attrs_list.push([0, 0, 0, 0, 0, 0]);
        }

        Self {
            selecting: 0,
            attrs_list,
        }
    }

    pub fn attrs(&self) -> Option<&[i32; 6]> {
        self.attrs_list.get(self.selecting)
    }

    pub fn attrs_mut(&mut self) -> Option<&mut [i32; 6]> {
        self.attrs_list.get_mut(self.selecting)
    }
}
