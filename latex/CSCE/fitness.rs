 pub fn fitness(&self) -> i32 {
        let mut fitness = 0f64;
        // The image we are trying to approximate.
        let goal = load_image(&self.filename);
        let rendered_strokes_buffer = self.render_strokes();
        for x in 0..goal.width() {
            for y in 0..goal.height() {
                let grgb = goal.get_pixel(x, y).data;
                let rrgb = rendered_strokes_buffer.get_pixel(x, y);
                let unfitness = (grgb[0] as i32 - rrgb[0] as i32).abs() +
                    (grgb[1] as i32 - rrgb[1] as i32).abs() +
                    (grgb[2] as i32 - rrgb[2] as i32).abs();
                fitness += 765.0 - unfitness as f64;
            }
        }
        return fitness as i32;
    }
}