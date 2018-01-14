pub struct Rectangle {
    l : u32,
    w : u32
}

impl Rectangle {
    pub fn new(l : u32, w : u32) -> Rectangle {
        Rectangle { l : l, w : w}
    }

    pub fn can_fit(&self, other : &Rectangle) -> bool {
        if self.l > other.l && self.w > other.w {
            true
        } else {
            false
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_fit_smaller() {
        let rect1 : Rectangle = Rectangle::new(2, 3);
        let rect2 : Rectangle = Rectangle::new(1, 1);
        assert!(rect1.can_fit(&rect2))
    }

    #[test]
    fn cannot_fit_larger() {
        let rect1 : Rectangle = Rectangle::new(2, 3);
        let rect2 : Rectangle = Rectangle::new(1, 1);
        assert!(rect2.can_fit(&rect1) == false)
    }
}
