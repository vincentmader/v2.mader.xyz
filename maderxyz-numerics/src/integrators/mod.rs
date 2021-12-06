
pub mod euler_explicit;
pub mod euler_implicit;
pub mod runge_kutta_2;
pub mod runge_kutta_4;
pub mod leap_frog;
pub mod verlet;


#[derive(Clone)]
pub enum Integrator {
    EulerExplicit,
    // EulerImplicit,
    // RungeKutta2,
    // RungeKutta4,
    // LeapFrog,
    // Verlet,
}


#[derive(Clone)]
pub enum FieldIntegrator {
    // TODO 
    Ising,
    Diffusion,
    GameOfLife,
}

