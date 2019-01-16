import UIKit

@UIApplicationMain
class AppDelegate : UIResponder, UIApplicationDelegate {
  var window : UIWindow?

  func application(_ application: UIApplication, didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey : Any]? = nil) -> Bool {
    let jsCodeLocation: URL?

    #if DEBUG
      jsCodeLocation = RCTBundleURLProvider.sharedSettings()?.jsBundleURL(forBundleRoot: "index", fallbackResource: nil)
    #else
      jsCodeLocation = Bundle.main.url(forResource: "main", withExtension: "jsbundle")
    #endif

    let rootView: RCTRootView = RCTRootView(bundleURL: jsCodeLocation,
                                            moduleName: "SensorData",
                                            initialProperties: nil,
                                            launchOptions: launchOptions)
    rootView.backgroundColor = .white

    self.window = UIWindow(frame: UIScreen.main.bounds)
    let rootViewController = UIViewController()
    rootViewController.view = rootView
    self.window?.rootViewController = rootViewController
    self.window?.makeKeyAndVisible()
    return true
  }
}
