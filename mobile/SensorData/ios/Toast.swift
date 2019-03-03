import Foundation

@objc(Toast)
class Toast: NSObject {
  @objc
  static func requiresMainQueueSetup() -> Bool {
    return true
  }

  @objc
  func constantsToExport() -> [AnyHashable : Any]! {
    return [:]
  }

  @objc
  func show(_ message: NSString, duration: NSNumber) {
    let alert = UIAlertController(title: nil, message: message as String, preferredStyle: .actionSheet)

    DispatchQueue.main.async {
      let controller = UIApplication.shared.delegate!.window!!.rootViewController!
      controller.present(alert, animated: true)
   }

    DispatchQueue.main.asyncAfter(deadline: DispatchTime.now() + Double(truncating: duration)) {
      alert.dismiss(animated: true)
    }
  }
}
