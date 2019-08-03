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

      if call.method == "getAcceleration" {
        self?.receiveAcceleration(result: result)
        return
      }

      if call.method == "getGravity" {
        self?.receiveGravity(result: result)
        return
      }

      if call.method == "getAttitude" {
        self?.receiveAttitude(result: result)
        return
      }

      if call.method == "getRotationRate" {
        self?.receiveRotationRate(result: result)
        return
      }

      if call.method == "getMagneticField" {
        self?.receiveMagneticField(result: result)
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
        result(FlutterError(code: "RELATIVE_ALTITUDE_UPDATE_ERROR", message: error.localizedDescription, details: nil))
        return
      }

      result(data!.pressure.doubleValue * 10.0)
    }
  }

  private func receiveAcceleration(result: @escaping FlutterResult) {
    let motionManager = CMMotionManager()

    motionManager.startAccelerometerUpdates(to: .main) { (data, error) in
      motionManager.stopAccelerometerUpdates()

      if let error = error {
        result(FlutterError(code: "ACCELEROMETER_UPDATE_ERROR", message: error.localizedDescription, details: nil))
        return
      }

      let acceleration = data!.acceleration
      result([acceleration.x, acceleration.y, acceleration.z])
    }
  }

  private func receiveRotationRate(result: @escaping FlutterResult) {
    let motionManager = CMMotionManager()

    motionManager.startGyroUpdates(to: .main) { (data, error) in
      motionManager.stopGyroUpdates()

      if let error = error {
        result(FlutterError(code: "GYROSCOPE_UPDATE_ERROR", message: error.localizedDescription, details: nil))
        return
      }

      let rotationRate = data!.rotationRate
      result([rotationRate.x, rotationRate.y, rotationRate.z])
    }
  }

  private func receiveMagneticField(result: @escaping FlutterResult) {
    let motionManager = CMMotionManager()

    motionManager.startMagnetometerUpdates(to: .main) { (data, error) in
      motionManager.stopMagnetometerUpdates()

      if let error = error {
        result(FlutterError(code: "MAGNETOMETER_UPDATE_ERROR", message: error.localizedDescription, details: nil))
        return
      }

      let magneticField = data!.magneticField
      result([magneticField.x, magneticField.y, magneticField.z])
    }
  }

  private func receiveGravity(result: @escaping FlutterResult) {
    let motionManager = CMMotionManager()

    motionManager.startDeviceMotionUpdates(to: .main) { (data, error) in
      motionManager.stopDeviceMotionUpdates()

      if let error = error {
        result(FlutterError(code: "DEVICEMOTION_UPDATE_ERROR", message: error.localizedDescription, details: nil))
        return
      }

      let gravity = data!.gravity
      result([gravity.x, gravity.y, gravity.z])
    }
  }

  private func receiveAttitude(result: @escaping FlutterResult) {
    let motionManager = CMMotionManager()

    motionManager.startDeviceMotionUpdates(to: .main) { (data, error) in
      motionManager.stopDeviceMotionUpdates()

      if let error = error {
        result(FlutterError(code: "DEVICEMOTION_UPDATE_ERROR", message: error.localizedDescription, details: nil))
        return
      }

      let attitude = data!.attitude
      result([attitude.roll, attitude.pitch, attitude.yaw])
    }
  }
}
