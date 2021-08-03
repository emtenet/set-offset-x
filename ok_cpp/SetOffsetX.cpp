#define NOMINMAX
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <wrl.h>
#include <d3d11_2.h>
#include <d2d1_2.h>
#include <dcomp.h>

#pragma comment(lib, "d3d11")
#pragma comment(lib, "d2d1")
#pragma comment(lib, "dcomp")

using namespace Microsoft::WRL;

int APIENTRY wWinMain(
    _In_ HINSTANCE hInstance,
    _In_opt_ HINSTANCE hPrevInstance,
    _In_ LPWSTR    lpCmdLine,
    _In_ int       nCmdShow)
{
    UNREFERENCED_PARAMETER(hInstance);
    UNREFERENCED_PARAMETER(hPrevInstance);
    UNREFERENCED_PARAMETER(lpCmdLine);
    UNREFERENCED_PARAMETER(nCmdShow);

    HRESULT result;

    ComPtr<ID3D11Device> device3d;
    result = D3D11CreateDevice(nullptr,
        D3D_DRIVER_TYPE_HARDWARE,
        nullptr,
        D3D11_CREATE_DEVICE_BGRA_SUPPORT,
        nullptr, 0,
        D3D11_SDK_VERSION,
        device3d.GetAddressOf(),
        nullptr,
        nullptr);
    if (S_OK != result)
    {
        OutputDebugString(L"ERROR in D3D11CreateDevice\n");
        return 0;
    }

    ComPtr<IDXGIDevice> deviceX;
    result = device3d.As(&deviceX);
    if (S_OK != result)
    {
        OutputDebugString(L"ERROR in cast to IDXGIDevice\n");
        return 0;
    }

    ComPtr<ID2D1Device> device2d;
    result = D2D1CreateDevice(deviceX.Get(), nullptr, device2d.GetAddressOf());
    if (S_OK != result)
    {
        OutputDebugString(L"ERROR in D2D1CreateDevice\n");
        return 0;
    }

    ComPtr<IDCompositionDesktopDevice> device;
    result = DCompositionCreateDevice2(
        device2d.Get(),
        __uuidof(device),
        reinterpret_cast<void**>(device.ReleaseAndGetAddressOf()));
    if (S_OK != result)
    {
        OutputDebugString(L"ERROR in DCompositionCreateDevice2\n");
        return 0;
    }

    ComPtr<IDCompositionVisual2> visual;
    result = device->CreateVisual(visual.GetAddressOf());
    if (S_OK != result)
    {
        OutputDebugString(L"ERROR in CreateVisual\n");
        return 0;
    }

    result = visual->SetOffsetX(205088.0);
    if (S_OK != result)
    {
        OutputDebugString(L"ERROR in SetOffsetX\n");
        return 0;
    }

    OutputDebugString(L"SUCCESS\n");

    return 0;
}
