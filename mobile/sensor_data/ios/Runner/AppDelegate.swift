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
    let sensorChannel = FlutterMethodChannel(name: "sensor_data.flutter.dev/sensor", binaryMessenger: controller)

    let sensors = Sensors()

    sensorChannel.setMethodCallHandler {
      [weak self] (call: FlutterMethodCall, result: @escaping FlutterResult) -> Void in
      if call.method == "getSensorInfo" {
        result(sensors.toDict().map {
          let jsonData = try! JSONSerialization.data(withJSONObject: $0, options: .prettyPrinted)
          return String(data: jsonData, encoding: .utf8)!
        })
        return
      }

      result(FlutterMethodNotImplemented)
    }

    GeneratedPluginRegistrant.register(with: self)
    return super.application(application, didFinishLaunchingWithOptions: launchOptions)
  }
}
