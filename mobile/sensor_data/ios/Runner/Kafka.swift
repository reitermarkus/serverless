import Foundation
import os.log

class Kafka {
  static func post(topic: String, records: [[String : Any]], completionHandler: (() -> Void)? = nil) -> URLSessionTask? {
    guard let host = UserDefaults.standard.string(forKey: "flutter.url") else {
      os_log("No URL set, cancelling send.")
      return nil
    }

    let url = URL(string: "\(host):8082/topics/\(topic)")!

    let json: [String : Any] = [
      "records": records
    ]

    let jsonData = try! JSONSerialization.data(withJSONObject: json, options: .prettyPrinted)
    let jsonString = String(data: jsonData, encoding: .utf8)!

    var request = URLRequest(url: url)
    request.setValue("application/vnd.kafka.json.v2+json", forHTTPHeaderField: "Content-Type")
    request.httpMethod = "POST"

    request.httpBody = jsonData

    os_log("%s", "POST \(url)")
    os_log("%s", "\(jsonString)")

    return URLSession.shared.dataTask(with: request) { (data, response, error) in
      completionHandler?()

      if let error = error {
        os_log("ERROR:")
        os_log("%s", "\(error.localizedDescription)")
      }

      if let data = data.flatMap({ String(data: $0, encoding: .utf8) }) {
        os_log("RESPONSE BODY:")
        os_log("%s", data)
      }
    }
  }
}
