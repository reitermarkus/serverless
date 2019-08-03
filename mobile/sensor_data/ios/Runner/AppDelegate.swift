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

    sensorChannel.setMethodCallHandler({
      [weak self] (call: FlutterMethodCall, result: @escaping FlutterResult) -> Void in
      if call.method == "getPressure" {
        result(sensors.pressure)
        return
      }

      if call.method == "getAcceleration" {
        result(sensors.acceleration.map { [$0.x, $0.y, $0.z] })
        return
      }

      if call.method == "getGravity" {
        result(sensors.gravity.map { [$0.x, $0.y, $0.z] })
        return
      }

      if call.method == "getAttitude" {
        result(sensors.attitude.map { [$0.roll, $0.pitch, $0.yaw] })
        return
      }

      if call.method == "getRotationRate" {
        result(sensors.rotationRate.map { [$0.x, $0.y, $0.z] })
        return
      }

      if call.method == "getMagneticField" {
        result(sensors.magneticField.map { [$0.x, $0.y, $0.z] })
        return
      }

      result(FlutterMethodNotImplemented)
    })
    GeneratedPluginRegistrant.register(with: self)
    return super.application(application, didFinishLaunchingWithOptions: launchOptions)
  }
}
