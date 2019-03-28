use core::f32;

pub type IIRState = [f32; 5];

#[derive(Default,Copy,Clone,Debug)]
pub struct IIR {
    pub x_offset: f32,
    pub y_offset: f32,
    pub ba: IIRState,
    pub scale: f32,
}

fn abs(x: f32) -> f32 {
    if x >= 0. { x } else { -x }
}

fn copysign(x: f32, y: f32) -> f32 {
    match () {
        _ if (x >= 0. && y >= 0.) || (x <= 0. && y <= 0.) => y,
        _ => -y
    }
}

impl IIR {
    pub fn pi(&mut self, kp: f32, ki: f32, g: f32) -> Result<(), &str> {
        let ki = copysign(kp, ki);
        let g = copysign(kp, g);
        let (a1, b0, b1) = match () {
            _ if abs(ki) < f32::EPSILON => (0., kp, 0.),
            _ => {
                let c = match () {
                    _ if abs(g) < f32::EPSILON => 1.,
                    _ => 1./(1. + ki/g)
                };
                let a1 = 2.*c - 1.;
                let b0 = ki*c + kp;
                let b1 = ki*c - a1*kp;
                if abs(b0 + b1) < f32::EPSILON {
                    return Err("low integrator gain and/or gain limit")
                }
                (a1, b0, b1)
            }
        };
        self.ba[0] = b0;
        self.ba[1] = b1;
        self.ba[2] = 0.;
        self.ba[3] = a1;
        self.ba[4] = 0.;
        Ok(())
    }

    pub fn update(&self, xy: &mut IIRState, x0: f32) -> f32 {
        xy.rotate_right(1);
        xy[0] = x0 + self.x_offset;
        let y0 = macc(self.y_offset, xy, &self.ba)
            .min(self.scale).max(-self.scale);
        xy[xy.len()/2] = y0;
        y0
    }
}

fn macc(y0: f32, x: &[f32], a: &[f32]) -> f32 {
    y0 + x.iter().zip(a.iter())
        .map(|(&i, &j)| i * j).sum::<f32>()
}