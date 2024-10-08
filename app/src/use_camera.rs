use math::CanvasPoint;
use yew::Reducible;

pub enum CameraStateAction {
    Refresh,
    MoveCamera {
        temp_canvas_position: CanvasPoint,
        offset: CanvasPoint,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct CameraState {
    canvas_position: CanvasPoint,
    zoom: f32,
}

impl CameraState {
    pub fn coord(&self) -> (f32, f32, f32) {
        let (x, y) = self.canvas_position.coord();

        (x, y, self.zoom)
    }

    pub fn canvas_position(&self) -> CanvasPoint {
        self.canvas_position
    }

    #[inline(always)]
    pub fn zoom_invalid(&self) -> bool {
        self.zoom == 0.0 || self.zoom.is_nan() || self.zoom.is_infinite()
    }

    pub fn zoom(&self) -> f32 {
        if self.zoom_invalid() {
            panic!("zoom is in invalid state")
        }

        self.zoom
    }

    pub fn convert_viewport_to_global(&self, other: CanvasPoint) -> CanvasPoint {
        if self.zoom_invalid() {
            panic!("zoom is in an invalid state before converting viewport to global");
        }

        other / CanvasPoint::new(self.zoom, self.zoom) - self.canvas_position
    }

    pub fn convert_global_to_viewport(&self, other: CanvasPoint) -> CanvasPoint {
        if self.zoom_invalid() {
            panic!("zoom is in an invalid state");
        }

        (other + self.canvas_position) * CanvasPoint::new(self.zoom, self.zoom)
    }
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            canvas_position: CanvasPoint::new(0.0, 0.0),
            zoom: 1.0,
        }
    }
}

impl Reducible for CameraState {
    type Action = CameraStateAction;
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            CameraStateAction::MoveCamera {
                temp_canvas_position,
                offset,
            } => {
                return Self {
                    canvas_position: temp_canvas_position + offset,
                    zoom: self.zoom,
                }
                .into();
            }
            CameraStateAction::Refresh => {
                return Self {
                    canvas_position: self.canvas_position,
                    zoom: self.zoom,
                }
                .into();
            }
        }
    }
}
