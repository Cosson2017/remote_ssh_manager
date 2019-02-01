trait Model {
    fn get_line(&self) -> Vec<&str>;
}

trait Widget {
    fn draw(&self);
    fn event(&mut self, key: char);
}

enum State {
    Normal,
    Search(String, String), // (left text,right text)
}

struct MainWidget {
    state: State,
}

impl Widget for MainWidget {
    fn draw(&self) {}

    fn event(&mut self, key: char) {
        use State::*;
        match self.state {
            Normal => self.event_normal(key),
            Search(..) => self.evnet_search(key),
        }
    }
}

impl MainWidget {
    fn new() -> Self {
        Self {
            state: State::Normal,
        }
    }
    fn event_normal(&mut self, key: char) {
        use State::*;
        match key {
            '/' => self.state = Search("/".into(), "".into()),
            '\t' => {}
            _ => {}
        }
    }

    fn evnet_search(&mut self, key: char) {
        use State::*;
        match key {
            '\n' => {}
            '\t' => {}
            '\x1B' => {
                //'esc'
            }
            // '->' => {}
            // '<-' => {}
            // 'esc' => {}
            _ => {
                if let Search(ref mut front, ..) = self.state {
                    front.push(key);
                }
            }
        }
    }
}
