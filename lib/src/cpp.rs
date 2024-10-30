#[cxx::bridge]
mod ffi {
    // C++に公開するRustの構造体
    #[derive(Debug)]
    pub struct Point {
        x: i32,
        y: i32,
    }

    extern "Rust" {
        // Pointに生やすメソッドを宣言
        fn new_point(x: i32, y: i32) -> Point;
        fn move_point(self: &Point, dx: i32, dy: i32) -> Point;
        fn display(self: &Point);
    }
}

// Pointを生成する関数
pub fn new_point(x: i32, y: i32) -> ffi::Point {
    ffi::Point { x, y }
}

impl ffi::Point {
    // Pointを移動させるメソッド
    pub fn move_point(&self, dx: i32, dy: i32) -> Self {
        ffi::Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    // Pointの情報を表示するメソッド
    pub fn display(&self) {
        println!("Point: ({}, {})", self.x, self.y);
    }
}
