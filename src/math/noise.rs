use super::vec::Vec3;

/// Hash continuo
#[inline] fn hash1(n: f32) -> f32 {
    (n.sin()*43758.5453).fract()
}

/// Hash 3D / escalar
#[inline] fn hash3(p: Vec3) -> f32 {
    let n = p.x*127.1 + p.y*311.7 + p.z*74.7;
    hash1(n)
}

/// Interpolación 
#[inline] fn smooth(t: f32) -> f32 { t*t*t*(t*(t*6.0 - 15.0) + 10.0) }

/// Lerp
#[inline] fn mix(a: f32, b: f32, t: f32) -> f32 { a + (b - a)*t }

/// Value noise 3D
pub fn noise3(p: Vec3) -> f32 {
    let i = Vec3::new(p.x.floor(), p.y.floor(), p.z.floor());
    let f = Vec3::new(p.x - i.x, p.y - i.y, p.z - i.z);
    let u = Vec3::new(smooth(f.x), smooth(f.y), smooth(f.z));

    let n000 = hash3(i);
    let n100 = hash3(Vec3::new(i.x+1.0, i.y,     i.z    ));
    let n010 = hash3(Vec3::new(i.x,     i.y+1.0, i.z    ));
    let n110 = hash3(Vec3::new(i.x+1.0, i.y+1.0, i.z    ));
    let n001 = hash3(Vec3::new(i.x,     i.y,     i.z+1.0));
    let n101 = hash3(Vec3::new(i.x+1.0, i.y,     i.z+1.0));
    let n011 = hash3(Vec3::new(i.x,     i.y+1.0, i.z+1.0));
    let n111 = hash3(Vec3::new(i.x+1.0, i.y+1.0, i.z+1.0));

    let nx00 = mix(n000, n100, u.x);
    let nx10 = mix(n010, n110, u.x);
    let nx01 = mix(n001, n101, u.x);
    let nx11 = mix(n011, n111, u.x);

    let nxy0 = mix(nx00, nx10, u.y);
    let nxy1 = mix(nx01, nx11, u.y);

    mix(nxy0, nxy1, u.z) 
}

/// Fractal Brownian Motion 
pub fn fbm(mut p: Vec3, octaves: i32, lacunarity: f32, gain: f32) -> f32 {
    let mut amp = 0.5;
    let mut sum = 0.0;
    for _ in 0..octaves {
        sum += amp * noise3(p);
        p = Vec3::new(p.x*lacunarity, p.y*lacunarity, p.z*lacunarity);
        amp *= gain;
    }
    sum
}