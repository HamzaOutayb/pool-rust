#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        const GRAVITY: f32 = 9.8;

        self.time += 1.0;

        let t = self.time;
        let x = self.init_position.x + self.init_velocity.x * t;
        let y = self.init_position.y + self.init_velocity.y * t - 0.5 * GRAVITY * t * t;

        let vx = self.init_velocity.x;
        let vy = self.init_velocity.y - GRAVITY * t;

        self.actual_position = Object { x:format!("{:.2}",x).parse::<f32>().unwrap(), y:format!("{:.2}",y).parse::<f32>().unwrap() };
        self.actual_velocity = Object { x: format!("{:.2}",vx).parse::<f32>().unwrap(), y: format!("{:.2}",vy).parse::<f32>().unwrap() };

        if y <= 0.0 {
            None
        } else {
            Some(self.clone())
        }
    }
}
