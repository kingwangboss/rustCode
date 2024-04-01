pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // trait对象，使用dyn关键字
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw Botton, width = {}, height = {}, label = {}",
        self.width, self.height, self.label);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub option: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw SelectBox, width = {}, height = {}, option = {:?}",
        self.width, self.height, self.option);
    }
}