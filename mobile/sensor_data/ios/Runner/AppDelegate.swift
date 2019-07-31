import UIKit
import Flutter
import CoreMotion

@UIApplicationMain
@objc class AppDelegate: FlutterAppDelegate {
  override func application(
    _ application: UIApplication,
    didFinishLaunchingWithOptions launchOptions: [UIApplicationLaunchOptionsKey: Any]?
  ) -> Bool {
    let controller : FlutterViewController = window?.rootViewController as! FlutterViewController
    let batteryChannel = FlutterMethodChannel(name: "sensor_data.flutter.dev/sensor",
                                              binaryMessenger: controller)
    
    batteryChannel.setMethodCallHandler({
      [weak self] (call: FlutterMethodCall, result: @escaping FlutterResult) -> Void in
      // Note: this method is invoked on the UI thread.
      if call.method == "getBatteryLevel" {
        self?.receiveBatteryLevel(result: result)
        return
      }
      
      if call.method == "getPressure" {
        self?.receivePressure(result: result)
        return
      }
      
      result(FlutterMethodNotImplemented)
    })
    GeneratedPluginRegistrant.register(with: self)
    return super.application(application, didFinishLaunchingWithOptions: launchOptions)
  }
  
  private func receiveBatteryLevel(result: FlutterResult) {
    let device = UIDevice.current
    device.isBatteryMonitoringEnabled = true
    if device.batteryState == UIDeviceBatteryState.unknown {
      result(FlutterError(code: "UNAVAILABLE",
                          message: "Battery info unavailable",
                          details: nil))
    } else {
      result(Int(device.batteryLevel * 100))
    }
  }
  
  
  private func receivePressure(result: @escaping FlutterResult) {
    let altimeter = CMAltimeter()
    
    altimeter.startRelativeAltitudeUpdates(to: .main) { (data, error) in
      altimeter.stopRelativeAltitudeUpdates()
      
      if let error = error {
        result(FlutterError(code: "RELATIVE_ALTITUDE_UPDATES_ERROR", message: error.localizedDescription, details: nil))
        return
      }
      
      result(data!.pressure.doubleValue * 10.0)
    }
  }
}
