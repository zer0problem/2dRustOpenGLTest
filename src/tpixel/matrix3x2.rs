use crate::tpixel::vector2::Vector2;

#[derive(Copy, Clone)]
pub struct Matrix3x2 {
    // 0, 1
    // 2, 3
    // 4, 5
    pub elements : [f32; 6],
}

impl Matrix3x2 {
    pub fn new() -> Matrix3x2 {
        Matrix3x2 { elements : [
            1.0f32, 0.0f32,
            0.0f32, 1.0f32,
            0.0f32, 0.0f32,]
        }
    }
    pub fn new_transform(position : Vector2, scale : Vector2, rotation_radians : f32) -> Matrix3x2 {
        let c = rotation_radians.cos();
        let s = rotation_radians.sin();
        Matrix3x2 { elements : [
             c * scale.x,  s * scale.x,
            -s * scale.y,  c * scale.y,
            position.x, position.y,]
        }
    }
    pub fn new_translation(position : Vector2) -> Matrix3x2 {
        Matrix3x2 {
            elements : [
                1f32, 0f32,
                0f32, 1f32,
                position.x, position.y,
            ]
        }
    }
    pub fn new_scale(scale : Vector2) -> Matrix3x2 {
        Matrix3x2 {
            elements : [
                scale.x, 0f32,
                0f32, scale.y,
                0f32, 0f32,
            ]
        }
    }
    pub fn new_rotation(rotation_radians : f32) -> Matrix3x2 {
        let c = rotation_radians.cos();
        let s = rotation_radians.sin();
        
        Matrix3x2 {
            elements : [
                c, s,
                -c, s,
                0f32, 0f32,
            ]
        }
    }
    pub fn inverse(&self) -> Matrix3x2 {
        // todo inverse scale....
        let rot = Matrix3x2 {
            elements : [
                // rotation
                self.elements[0], self.elements[2],
                self.elements[1], self.elements[3],
                // position
                0f32, 0f32,
            ]
        };
        let pos = Matrix3x2::new_transform(Vector2{x:-self.elements[4],y:-self.elements[5]}, Vector2{x:1f32,y:1f32}, 0f32);
        Matrix3x2::mul(&pos, &rot)
    }
    pub fn translate(&mut self, translation : Vector2) {
        let tra_mat = Matrix3x2::new_transform(translation, Vector2 {x:1f32, y:1f32}, 0f32);
        let cop_mat = *self;
        *self = Matrix3x2::mul(&cop_mat, &tra_mat);
    }
    pub fn rotate(&mut self, rotation_radians : f32) {
        let rot_mat = Matrix3x2::new_transform(Vector2::new(), Vector2 {x:1f32, y:1f32}, rotation_radians);
        let cop_mat = *self;
        *self = Matrix3x2::mul(&rot_mat, &cop_mat);
    }
    pub fn get_position(&self) -> Vector2 {
        Vector2 {
            x : self.elements[4],
            y : self.elements[5],
        }
    }
    fn mul(matrix_a : &Matrix3x2, matrix_b : &Matrix3x2) -> Matrix3x2 {
        let a = &matrix_a.elements;
        let b = &matrix_b.elements;
        Matrix3x2 {
            elements : [
                a[0] * b[0] + a[1] * b[2]       , a[0] * b[1] + a[1] * b[3],
                a[2] * b[0] + a[3] * b[2]       , a[2] * b[1] + a[3] * b[3],
                a[4] * b[0] + a[5] * b[2] + b[4], a[4] * b[1] + a[5] * b[3] + b[5],
            ]
        }
    }
}
