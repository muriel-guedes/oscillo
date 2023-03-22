use std::sync::{Arc, Mutex};

use wgpu::RenderPass;

use crate::{Coord, WindowSize, Background, CoordValue, BackgroundType, Position};

#[derive(Clone, Default)]
pub struct Element {
    pub style: Arc<Style>,
    parent: Arc<Mutex<Option<Element>>>,
    children: Arc<Mutex<Vec<Element>>>
}
impl Element {
    pub(crate) fn set_scissor_rect(&self, render_pass: &mut RenderPass, ws: WindowSize) {
        render_pass.set_scissor_rect(
            self.get_x(ws) as u32,
            self.get_y(ws) as u32,
            self.get_width(ws) as u32,
            self.get_height(ws) as u32
        )
    }

    pub fn add_child(&self, child: Self) {
        self.children.lock().unwrap().push(child)
    }
    pub fn create_child(&self) -> Self {
        let child = Self::default();
        child.set_parent(Some(self.clone()));
        self.add_child(child.clone());
        child
    }

    pub fn get_parent(&self) -> Option<Element> {
        self.parent.lock().unwrap().clone()
    }
    pub fn set_parent(&self, parent: Option<Element>) {
        *self.parent.lock().unwrap() = parent
    }

    fn get_data_loop(&self, ws: WindowSize, data: &mut Vec<f32>) {
        match self.style.background.get() {
            BackgroundType::Solid(bg) => {
                if bg.a > 0. {
                    let x = self.get_x(ws);
                    let y = self.get_y(ws);
                    let width = self.get_width(ws);
                    let height = self.get_height(ws);
                    data.append(&mut vec![
                        1., x, x + width, y, y+height,
                        bg.r, bg.g, bg.b, bg.a
                    ])
                }
            }
            BackgroundType::None => {}
        }
        for child in self.children.lock().unwrap().iter() {
            child.get_data_loop(ws, data)
        }
    }
    pub(crate) fn get_data(&self, ws: WindowSize) -> Vec<f32> {
        let mut data = Vec::new();
        self.get_data_loop(ws, &mut data);
        data
    }

    pub fn get_content_width(&self) -> f32 {
        let mut res = 0.;
        for child in self.children.lock().unwrap().iter() {
            res += match child.style.width.get() {
                CoordValue::Px(v) => v,
                CoordValue::Perc(_) => 0.,
                CoordValue::Auto => self.get_content_width()
            }
        }
        res
    }
    pub fn get_content_height(&self) -> f32 {
        let mut res = 0.;
        for child in self.children.lock().unwrap().iter() {
            res += match child.style.height.get() {
                CoordValue::Px(v) => v,
                CoordValue::Perc(_) => 0.,
                CoordValue::Auto => self.get_content_height()
            }
        }
        res
    }
    pub fn get_width(&self, ws: WindowSize) -> f32 {
        match self.style.width.get() {
            CoordValue::Px(v) => v,
            CoordValue::Perc(v) => match self.get_parent() {
                Some(parent) => parent.get_width(ws) * v,
                None => ws.width * v
            }
            CoordValue::Auto => self.get_content_width()
        }
    }
    pub fn get_height(&self, ws: WindowSize) -> f32 {
        match self.style.height.get() {
            CoordValue::Px(v) => v,
            CoordValue::Perc(v) => match self.get_parent() {
                Some(parent) => parent.get_height(ws) * v,
                None => ws.height * v
            }
            CoordValue::Auto => self.get_content_height()
        }
    }
    pub fn get_x(&self, ws: WindowSize) -> f32 {
        match self.style.x.get() {
            CoordValue::Px(v) => if self.style.position.is_absolute() {
                v
            } else {
                match self.get_parent() {
                    Some(parent) => parent.get_x(ws) + v,
                    None => v
                }
            },
            CoordValue::Perc(v) => match self.get_parent() {
                Some(parent) => parent.get_width(ws) * v + parent.get_x(ws),
                None => ws.width * v
            }
            CoordValue::Auto => match self.get_parent() {
                Some(parent) => parent.get_x(ws),
                None => 0.
            }
        }
    }
    pub fn get_y(&self, ws: WindowSize) -> f32 {
        match self.style.y.get() {
            CoordValue::Px(v) => if self.style.position.is_absolute() {
                v
            } else {
                match self.get_parent() {
                    Some(parent) => parent.get_y(ws) + v,
                    None => v
                }
            },
            CoordValue::Perc(v) => match self.get_parent() {
                Some(parent) => parent.get_height(ws) * v + parent.get_y(ws),
                None => ws.height * v
            }
            CoordValue::Auto => match self.get_parent() {
                Some(parent) => parent.get_y(ws),
                None => 0.
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct Style {
    pub width: Coord,
    pub height: Coord,
    pub x: Coord,
    pub y: Coord,
    pub background: Background,
    pub position: Position
}