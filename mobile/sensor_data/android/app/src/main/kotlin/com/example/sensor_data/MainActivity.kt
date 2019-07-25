package com.sensor_data

import android.os.Bundle
import android.util.Log

import io.flutter.app.FlutterActivity
import io.flutter.plugins.GeneratedPluginRegistrant
import io.flutter.plugin.common.MethodChannel
import io.flutter.plugin.common.BasicMessageChannel
import io.flutter.plugin.common.StringCodec

import org.json.JSONObject


class MainActivity: FlutterActivity() {
  private val CPU_CHANNEL = "sensor_data.flutter.dev/cpu_info"
  private val SERVICE_CHANNEL = "sensor_data.flutter.dev/service"
  private val SENSOR_CHANNEL = "sensor_data.flutter.dev/sensor"
  private val SETTINGS_CHANNEL = "sensor_data.flutter.dev/settings"
  private var sensorServiceModule : SensorServiceModule = SensorServiceModule()

  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)

    GeneratedPluginRegistrant.registerWith(this)
    MethodChannel(flutterView, CPU_CHANNEL).setMethodCallHandler { call, result ->
      if (call.method.equals("getCpuInfo")) {
        val cpuInfo : JSONObject? = CpuInfo.asJson()

        if (cpuInfo != null) {
          result.success(cpuInfo.toString())
        } else {
          result.error("UNAVAILABLE", "CPU Info not available.", null);
        }
      } else {
        result.notImplemented()
      }
    }

    MethodChannel(flutterView, SERVICE_CHANNEL).setMethodCallHandler { call, result ->
      val channel = BasicMessageChannel<String>(flutterView, "sensor", StringCodec.INSTANCE)

      if (call.method.equals("startService")) {
        val url : String? = call.argument("url")

        val res = sensorServiceModule.startService(url!!, getApplicationContext(), {
          jBody -> channel.send(jBody.toString())
        })
        if (res.first) {
          result.success("successfully started service.")
        } else {
          result.error("UNAVAILABLE", res.second, null)
        }
      } else {
        result.notImplemented()
      }
    }

    MethodChannel(flutterView, SENSOR_CHANNEL).setMethodCallHandler { call, result ->
      if (call.method.equals("getSensorInfo")) {
        val sensorInfo : JSONObject? = Sensors.getInstance(getApplicationContext()).asJson()

        if (sensorInfo != null) {
          result.success(sensorInfo.toString())
        } else {
          result.error("UNAVAILABLE", "Sensor Info not available.", null);
        }
      } else {
        result.notImplemented()
      }
    }

    MethodChannel(flutterView, SETTINGS_CHANNEL).setMethodCallHandler { call, result ->
      if (call.method.equals("changeSettings")) {
        val interval : Int? = call.argument("interval")
        val url : String? = call.argument("url")

        sensorServiceModule.updateInterval = interval!!
        sensorServiceModule.url = url!!

        result.success("applied \"${sensorServiceModule.updateInterval}\" for interval and ${sensorServiceModule.url} for url.")
      } else {
        result.notImplemented()
      }
    }
  }
}
