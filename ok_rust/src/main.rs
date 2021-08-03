use std::ptr::null_mut;

use winapi::{
    ctypes::c_void,
    shared::dxgi::IDXGIDevice,
    shared::winerror::S_OK,
    um::d2d1_1::{D2D1CreateDevice, ID2D1Device},
    um::d3d11::{
        D3D11CreateDevice, ID3D11Device, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION,
    },
    um::d3dcommon::D3D_DRIVER_TYPE_HARDWARE,
    um::dcomp::{DCompositionCreateDevice2, IDCompositionDesktopDevice, IDCompositionVisual2},
    um::unknwnbase::IUnknown,
    Interface,
};

use wio::com::ComPtr;

fn main() {
    unsafe {
        let mut device3d: *mut ID3D11Device = null_mut();
        let result = D3D11CreateDevice(
            null_mut(),
            D3D_DRIVER_TYPE_HARDWARE,
            null_mut(),
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            null_mut(),
            0,
            D3D11_SDK_VERSION,
            &mut device3d,
            null_mut(),
            null_mut(),
        );
        if result != S_OK {
            eprintln!("HRESULT {} in D3D11CreateDevice", result);
            return;
        }
        let device3d = ComPtr::from_raw(device3d);

        let device_x: ComPtr<IDXGIDevice> = device3d.cast().unwrap();

        let mut device2d: *mut ID2D1Device = null_mut();
        let result = D2D1CreateDevice(device_x.as_raw(), null_mut(), &mut device2d);
        if result != S_OK {
            eprintln!("HRESULT {} in D2D1CreateDevice", result);
            return;
        }
        let device2d = ComPtr::from_raw(device2d);
        let device2d_iunknown: ComPtr<IUnknown> = device2d.cast().unwrap();

        let mut device: *mut IDCompositionDesktopDevice = null_mut();
        let result = DCompositionCreateDevice2(
            device2d_iunknown.as_raw(),
            &IDCompositionDesktopDevice::uuidof(),
            &mut device as *mut *mut _ as *mut *mut c_void,
        );
        if result != S_OK {
            eprintln!("HRESULT {} in DCompositionCreateDevice2", result);
            return;
        }
        let device = ComPtr::from_raw(device);

        let mut visual: *mut IDCompositionVisual2 = null_mut();
        let result = device.CreateVisual(&mut visual);
        if result != S_OK {
            eprintln!("HRESULT {} in CreateVisual", result);
            return;
        }
        let visual = ComPtr::from_raw(visual);

        let result = visual.SetOffsetX_1(0.0);
        if result != S_OK {
            eprintln!("HRESULT {} in SetOffsetX", result);
            return;
        }
    }

    eprintln!("SUCCESS");
}
