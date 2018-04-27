# Structs

```rust
struct Rectangle {
    width u32
    height u32
}

impl Rectangle {
    // square constructor is an associated function.
    // let s = Rectangle::square(42);
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // area is a read only method.
    // let area = r.area();
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // squarify sets the dimensions to a largest square that can fit inside the rectangle.
    // squarify requires a mutable reference.
    fn squarify(mut &self) {
        if self.width > self.height {
            self.width = self.height
        } else {
            self.height = self.width
        }
    }
}
```