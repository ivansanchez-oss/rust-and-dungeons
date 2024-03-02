use wgpu::RequestDeviceError;

use wgpu::CreateSurfaceError;

#[derive(Debug)]
pub enum RenderError {
    Surface(CreateSurfaceError),
    Device(RequestDeviceError),
    FoundAdapaterError,
}

impl From<CreateSurfaceError> for RenderError {
    fn from(value: CreateSurfaceError) -> Self {
        RenderError::Surface(value)
    }
}

impl From<RequestDeviceError> for RenderError {
    fn from(value: RequestDeviceError) -> Self {
        RenderError::Device(value)
    }
}

impl std::error::Error for RenderError {}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
