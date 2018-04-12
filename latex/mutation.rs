
match rng.gen::<i32>() % 3 {
    0 => {
        to_modify.start.x = (to_modify.start.x 
                            + rng.gen::<u32>() % mutation_strength) % self.width;   
        to_modify.start.y = (to_modify.start.y
                            + rng.gen::<u32>() % mutation_strength) % self.height;  
    }
    1 => {
        to_modify.end.x = (to_modify.end.x 
                          + rng.gen::<u32>() % mutation_strength) % self.width;       
        to_modify.end.y = (to_modify.end.y
                          + rng.gen::<u32>() % mutation_strength) % self.height;      
    }
    2 => {
        to_modify.width = to_modify.width + rng.gen::<u32>() % mutation_strength;
    }
    _ => (),

}

s.strokes.remove(to_modify_index);
s.strokes.push(to_modify);
let post = s.fitness();
if post > pre {
    return s;
} else {
    return self.clone();
}

