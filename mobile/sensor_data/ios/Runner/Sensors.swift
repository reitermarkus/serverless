import Foundation
import Flutter
import CoreMotion
import os.log

class Sensors {
  let motionManager = CMMotionManager()
  let altimeter = CMAltimeter()

  var acceleration: CMAcceleration?
  var rotationRate: CMRotationRate?
  var magneticField: CMMagneticField?
  var gravity: CMAcceleration?
  var attitude: CMAttitude?
  var pressure: Double?

  var lastSend: Date?
  var currentTask: URLSessionTask?

  var queue = DispatchQueue.global(qos: .utility)
  var semaphore = DispatchSemaphore(value: 1)

  init() {
    self.altimeter.startRelativeAltitudeUpdates(to: .main) { (data, _) in
      self.pressure = data.map { $0.pressure.doubleValue * 10.0 }

      self.sendData()
    }

    self.motionManager.startAccelerometerUpdates(to: .main) { (data, _) in
      self.acceleration = data.map { $0.acceleration }

      self.sendData()
    }

    self.motionManager.startGyroUpdates(to: .main) { (data, _) in
      self.rotationRate = data.map { $0.rotationRate }

      self.sendData()
    }

    self.motionManager.startMagnetometerUpdates(to: .main) { (data, _) in
      self.magneticField = data.map { $0.magneticField }

      self.sendData()
    }

    self.motionManager.startDeviceMotionUpdates(to: .main) { (data, _) in
      self.gravity = data.map { $0.gravity }
      self.attitude = data.map { $0.attitude }

      self.sendData()
    }
  }

  func sendData() {
    self.semaphore.wait()
    
    defer {
      self.semaphore.signal()
    }

    guard self.currentTask == nil else { return }

    guard let acceleration = self.acceleration else { return }
    guard let rotationRate = self.rotationRate else { return }
    guard let magneticField = self.magneticField else { return }
    guard let gravity = self.gravity else { return }
    guard let attitude = self.attitude else { return }
    guard let pressure = self.pressure else { return }

    var intervalMs = UserDefaults.standard.double(forKey: "flutter.interval")
    intervalMs = intervalMs == 0 ? 10000 : intervalMs
    let interval = intervalMs / 1000.0

    if let lastSend = self.lastSend {
      if lastSend > Date().addingTimeInterval(-TimeInterval(interval)) {
        return
      }
    }

    lastSend = Date()

    guard let host = UserDefaults.standard.string(forKey: "flutter.url") else {
      os_log("No URL set, cancelling send.")
      return
    }

    let url = URL(string: "\(host):8082/topics/sensor")!

    let json: [String : Any] = [
      "records": [[
        "value": value(
          acceleration: acceleration,
          rotationRate: rotationRate,
          magneticField: magneticField,
          gravity: gravity,
          attitude: attitude,
          pressure: pressure
        ),
      ]]
    ]

    let jsonData = try! JSONSerialization.data(withJSONObject: json, options: .prettyPrinted)
    let jsonString = String(data: jsonData, encoding: .utf8)!

    os_log("%s", "POST \(url)")
    os_log("%s", "\(jsonString)")

    var request = URLRequest(url: url)
    request.setValue("application/vnd.kafka.json.v2+json", forHTTPHeaderField: "Content-Type")
    request.httpMethod = "POST"

    request.httpBody = jsonData

    self.currentTask = URLSession.shared.dataTask(with: request) { (data, response, error) in
      self.currentTask = nil
      self.lastSend = Date()

      if let error = error {
        os_log("ERROR:")
        os_log("%s", "\(error.localizedDescription)")
      }

      if let data = data.flatMap({ String(data: $0, encoding: .utf8) }) {
        os_log("RESPONSE BODY:")
        os_log("%s", data)
      }
    }
    self.currentTask!.resume()
  }
  

  func value(acceleration: CMAcceleration, rotationRate: CMRotationRate, magneticField: CMMagneticField, gravity: CMAcceleration, attitude: CMAttitude, pressure: Double) -> [String : Any] {
    return [
      "acceleration": [
        "x": acceleration.x,
        "y": acceleration.y,
        "z": acceleration.z,
      ],
      "rotation_rate": [
        "x": rotationRate.x,
        "y": rotationRate.y,
        "z": rotationRate.z,
      ],
      "magnetic_field": [
        "x": magneticField.x,
        "y": magneticField.y,
        "z": magneticField.z,
      ],
      "gravity": [
        "x": gravity.x,
        "y": gravity.y,
        "z": gravity.z,
      ],
      "attitude": [
        "roll": attitude.roll,
        "pitch": attitude.pitch,
        "yaw": attitude.yaw,
      ],
      "pressure": pressure
    ]
  }

  deinit {
    self.altimeter.stopRelativeAltitudeUpdates()
    self.motionManager.stopAccelerometerUpdates()
    self.motionManager.stopGyroUpdates()
    self.motionManager.stopDeviceMotionUpdates()
  }
}
