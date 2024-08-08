/*
pub struct EditorState {
    map: Map,
    cursor: Point,
    key: Option<VirtualKeyCode>,
    shift: bool,
    control: bool,
    brush_tile: Option<TileType>,
}
impl EditorState {
    fn new() -> Self {
        Self {
            map: Map::new(),
            cursor: Point::new(0, 0),
            key: None,
            shift: false,
            control: false,
            brush_tile: None,
        }
    }
}

impl GameState for EditorState {
    fn tick(&mut self, ctx: &mut BTerm) {
        //clear the screen
        ctx.cls();
        //capture user input
        (self.key, self.shift, self.control) = (ctx.key, ctx.shift, ctx.control);
    }
}
*/
