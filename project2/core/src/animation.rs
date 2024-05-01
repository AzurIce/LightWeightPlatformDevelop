use crate::render::BitmapAsset;

struct AnimationTicker {
    tick_per_frame: usize,
    tick_cnt: usize,

    pub total: usize,
    pub cur: usize,
}

impl AnimationTicker {
    pub fn new(tick_per_frame: usize, total: usize) -> Self {
        Self {
            tick_per_frame,
            tick_cnt: 0, 

            total,
            cur: 0
        }
    }
    
    pub fn tick(&mut self) {
        self.tick_cnt += 1;
        if self.tick_cnt == self.tick_per_frame {
            self.tick_cnt = 0;
            self.cur = (self.cur + 1) % self.total;
        }
    }
}

pub struct AnimatedBitmap {
    animation_tick: AnimationTicker,
    bitmaps: Vec<BitmapAsset>,
}

impl AnimatedBitmap {
    pub fn new(bitmaps: Vec<BitmapAsset>, tick_per_frame: usize) -> Self {
        Self {
            animation_tick: AnimationTicker::new(tick_per_frame, bitmaps.len()),
            bitmaps,
        }
    }

    pub fn tick(&mut self) {
        self.animation_tick.tick()
    }

    pub fn cur_bitmap(&self) -> BitmapAsset {
        self.bitmaps.get(self.animation_tick.cur).unwrap().clone()
    }
}