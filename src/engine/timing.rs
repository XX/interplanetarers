
/// time between current frame and last frame
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Timing {
    pub delta_time: f32,
    pub last_frame: f32,
}