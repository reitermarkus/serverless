import UIKit
import Flutter
import CoreMotion
import os.log

@UIApplicationMain
@objc class AppDelegate: FlutterAppDelegate {
  override func application(
    _ application: UIApplication,
    didFinishLaunchingWithOptions launchOptions: [UIApplicationLaunchOptionsKey: Any]?
  ) -> Bool {
    let controller : FlutterViewController = window?.rootViewController as! FlutterViewController
    let sensorChannel = FlutterMethodChannel(name: "sensor_data.flutter.dev/sensor", binaryMessenger: controller.binaryMessenger)

    let sensors = Sensors()

    sensorChannel.setMethodCallHandler { (call, result) in
      if call.method == "getSensorInfo" {
        result(sensors.toDict().map {
          let jsonData = try! JSONSerialization.data(withJSONObject: $0, options: .prettyPrinted)
          return String(data: jsonData, encoding: .utf8)!
        })
        return
      }

      result(FlutterMethodNotImplemented)
    }

    let uuid = UIDevice.current.identifierForVendor!.uuidString
    let deviceName = UIDevice.current.name

    Kafka.post(topic: "register-device", records: [["value": [
      "id": uuid,
      "name": deviceName,
    ]]])?.resume()

    GeneratedPluginRegistrant.register(with: self)
    return super.application(application, didFinishLaunchingWithOptions: launchOptions)
  }
}
