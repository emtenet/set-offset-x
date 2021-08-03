mod bindings {
    windows::include_bindings!();
}

use std::ptr::null_mut;

use bindings::{
    Windows::Win32::Graphics::Direct2D::D2D1CreateDevice,
    Windows::Win32::Graphics::Direct3D11::{
        D3D11CreateDevice, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION,
        D3D_DRIVER_TYPE_HARDWARE,
    },
    Windows::Win32::Graphics::DirectComposition::{
        DCompositionCreateDevice2, IDCompositionDesktopDevice,
    },
    Windows::Win32::Graphics::Dxgi::IDXGIDevice,
};

use windows::Interface;

fn main() {
    unsafe {
        let mut device3d = None;

        D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            None,
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            null_mut(),
            0,
            D3D11_SDK_VERSION,
            &mut device3d,
            null_mut(),
            null_mut(),
        )
        .unwrap();

        let device3d = device3d.unwrap();

        let device_x = device3d.cast::<IDXGIDevice>().unwrap();

        let device2d = D2D1CreateDevice(device_x, null_mut()).unwrap();

        let device: IDCompositionDesktopDevice = DCompositionCreateDevice2(device2d).unwrap();

        let visual = device.CreateVisual().unwrap();

        visual.SetOffsetX(0.0).unwrap();
    }

    eprintln!("SUCCESS");
}
