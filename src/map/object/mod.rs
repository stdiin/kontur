use super::viewer::MapViewer;

pub trait MapObject {
    fn render(&self, _viewer: &MapViewer);
}