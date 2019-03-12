#[derive(Default)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Default)]
struct Area {
    w: isize,
    h: isize,
}

#[derive(Default)]
struct Color {
    fg: String,
    bg: String
}

#[derive(Default)]
struct TextUnit<'a> {
    text: &'a str,
    pos: Pos,
    area: Area,
    color: Color,
}

impl<'a> TextUnit<'a> {
}

impl<'a> Widget for TextUnit<'a> {
    fn draw(&self) {

    }
}


trait Widget {
    fn draw(&self);
    fn event(&self) {}
}


trait TextBlock {
    fn text_unit_iter(&self) -> Vec<TextUnit>;

    fn get_pos(&self) -> Pos;
    fn get_area(&self) -> Area;
}

impl<T: TextBlock> Widget for T {
    fn draw(&self) {
        let Pos{x, y} = self.get_pos();
        let Area{w, h} = self.get_area();
        println!("{} {} {} {}", x, y, w, h);
        for line in self.text_unit_iter() {
            line.draw();
        }
    }

    fn event(&self) {

    }
}

