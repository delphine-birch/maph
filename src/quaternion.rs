use crate::vector::*;
pub fn euler_to_quaternion(r: Vector3<f32>) -> Vector4<f32> {
    let phi = r.x/2.0;
    let the = r.y/2.0;
    let psi = r.z/2.0;
    Vector4::new(
        phi.cos()*the.cos()*psi.cos() + phi.sin()*the.sin()*psi.sin(),
        phi.sin()*the.cos()*psi.cos() - phi.cos()*the.sin()*psi.sin(),
        phi.cos()*the.sin()*psi.cos() + phi.sin()*the.cos()*psi.sin(),
        phi.cos()*the.cos()*psi.sin() - phi.sin()*the.sin()*psi.cos()
    )
}
pub fn quaternion_to_euler(q: Vector4<f32>) -> Vector3<f32> {
    let t0 = 2.0*(q.w*q.x + q.y*q.z);
    let t1 = 1.0 - 2.0*(q.x*q.x + q.y*q.y);
    let mut t2 = 2.0*(q.w*q.y - q.z*q.x);
    t2 = t2.clamp(-1.0, 1.0);
    let t3 = 2.0*(q.w*q.z + q.x*q.y);
    let t4 = 1.0 - 2.0*(q.y*q.y - q.z*q.z);
    Vector3::new(t0.atan2(t1), t2.asin(), t3.atan2(t4))
}

pub fn axis_angle(ax: Vector3<f32>, a: f32) -> Vector4<f32> {
    let ax0 = ax / ax.mag();
    let a0 = ((a/2.0).sin(), (a/2.0).cos());
    Vector4::new(
        ax0.x*a0.0,
        ax0.y*a0.0,
        ax0.z*a0.0,
        a0.1
    )
}