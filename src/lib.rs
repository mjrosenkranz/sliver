use std::rc::Rc;

#[derive(Debug)]
struct Bounds (f32, f32, f32);

struct World {
    geoms: Vec<Geometric>
}

struct Build<'a> {
    bounds: Bounds,
    parts: Vec<Part<'a>>,
}

struct Part<'a> {
    geoms: Vec<&'a Geometric>
}

#[derive(Debug)]
struct Geometric {
    bounds: Bounds
}

impl Geometric {
    
    fn new() -> Self {
        Geometric{
            bounds: Bounds(0., 0., 0.),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut source: Vec<Box<Geometric>> = vec!(
            Box::new(Geometric::new()),
        );


        let mut part = Part {
            geoms: vec!()
        };

        for s in source.iter() {
            part.geoms.push(&**s);
        }

        // stops you from doing this!
        // source.remove(0);

        println!("{:?}", part.geoms[0]);
    }

}
