use nalgebra::Vector2;


// speed is pixel/tick
// and normally, the tick rate is 20tick/s
// so the speed is pixel/0.05s (pixel/50ms)

/// A struct of motion attribute constants used in motion calc
pub enum MotionAttribute {
    Static,
    UniformSpeed {
        speed: f32
    },
    Accelerated {
        acceleration: f32
    },
    AcceleratedWithFriction {
        acceleration: f32,
        friction: f32
    }
}