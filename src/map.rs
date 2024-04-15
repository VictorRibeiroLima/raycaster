use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
    Portal,
}

pub struct MapTile {
    pub rect: Rect,
    pub color: Color,
    pub tile_type: TileType,
}

pub struct Map {
    x: usize,
    y: usize,
    size: usize,
    raw_tiles: Vec<u8>,
    tiles: Vec<MapTile>,
    walls: Vec<Rect>,
}

impl Map {
    pub fn new(x: usize, y: usize, tiles: Vec<u8>) -> Map {
        let map_x = x;
        let map_y = y;
        let size = x * y;
        let len = tiles.len();
        let mut reacts = Vec::new();
        let mut walls = Vec::new();
        if len != size {
            panic!("Map size does not match x and y values");
        }

        for y in 0..map_y {
            for x in 0..map_x {
                let color: Color;
                let tile_type: TileType;
                let rect = Rect::new(
                    (x * size) as i32,
                    (y * size) as i32,
                    size as u32,
                    size as u32,
                );
                if tiles[y * map_x + x] == 1 {
                    color = Color::RGB(255, 0, 0);
                    tile_type = TileType::Wall;
                    walls.push(rect);
                } else {
                    color = Color::RGB(0, 0, 0);
                    tile_type = TileType::Floor;
                }

                let tile = MapTile {
                    rect,
                    color,
                    tile_type,
                };
                reacts.push(tile);
            }
        }

        Map {
            x,
            y,
            size,
            raw_tiles: tiles,
            tiles: reacts,
            walls,
        }
    }

    pub fn titles(&self) -> &Vec<MapTile> {
        &self.tiles
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for rect_with_color in &self.tiles {
            canvas.set_draw_color(rect_with_color.color);
            let _ = canvas.fill_rect(rect_with_color.rect);
        }
    }

    pub fn is_wall(&self, target: Rect) -> bool {
        for wall in &self.walls {
            if wall.has_intersection(target) {
                return true;
            }
        }
        false
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn tile_at(&self, i: usize) -> Option<&MapTile> {
        self.tiles.get(i)
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
