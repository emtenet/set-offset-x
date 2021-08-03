fn main() {
    windows::build! {
        Windows::Win32::Graphics::Direct2D::{
            D2D1CreateDevice,
            ID2D1Device,
        },
        Windows::Win32::Graphics::Direct3D11::{
            D3D11_CREATE_DEVICE_FLAG,
            D3D11_SDK_VERSION,
            D3D11CreateDevice,
            D3D_DRIVER_TYPE,
            ID3D11Device,
        },
        Windows::Win32::Graphics::DirectComposition::{
            DCompositionCreateDevice2,
            IDCompositionDesktopDevice,
            IDCompositionVisual2,
        },
        Windows::Win32::Graphics::Dxgi::{
            IDXGIDevice,
        },
    };
}
