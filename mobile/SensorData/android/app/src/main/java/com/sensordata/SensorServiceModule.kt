package com.sensordata

import android.support.annotation.Nullable
import com.facebook.react.bridge.ReactApplicationContext
import com.facebook.react.bridge.ReactContextBaseJavaModule
import com.facebook.react.bridge.ReactMethod
import com.facebook.react.bridge.WritableMap
import com.facebook.react.modules.core.DeviceEventManagerModule
import java.util.HashMap
import android.util.Log
import com.facebook.react.bridge.Promise
import android.content.Intent
import android.app.PendingIntent
import android.graphics.BitmapFactory
import com.sensordata.SensorService
import android.content.Context;
import android.hardware.SensorManager
import android.os.Handler

import org.json.JSONObject
import org.json.JSONArray

import com.sensorData.Sensors

class SensorServiceModule(context: ReactApplicationContext): ReactContextBaseJavaModule(context) {
  override fun getName() = "SensorService"

  @ReactMethod
  fun startService(promise: Promise) {
    Log.d(REACT_CLASS, "startService")

    try {
      val intent = Intent(FOREGROUND)
      intent.setClass(this.getReactApplicationContext(), SensorService::class.java)
      getReactApplicationContext().startService(intent)
      Log.d(REACT_CLASS, "startService, success")
      networkLoop()
      promise.resolve(true)
    }
    catch (e: Exception) {
      Log.d(REACT_CLASS, "startService failed!")
      promise.reject(e)
    }
  }

  @ReactMethod
  fun stopService(promise: Promise) {
    Log.d(REACT_CLASS, "stopService")

    try {
      val intent = Intent(FOREGROUND)
      intent.setClass(this.getReactApplicationContext(), SensorService::class.java)
      this.getReactApplicationContext().stopService(intent)
    }
    catch (e:Exception) {
      Log.d(REACT_CLASS, "stopService failed!")
      promise.reject(e)
    }

    promise.resolve(true)
  }

  private fun networkLoop() {
    val handler = Handler()

    val applicationContext = this.getReactApplicationContext()

    val manager = applicationContext.getSystemService(Context.SENSOR_SERVICE) as SensorManager
    val sensors = Sensors(manager)

    handler.postDelayed(object : Runnable {
      override fun run() {
        val jsonBody = JSONObject()
        val records = JSONObject()
        val recordsArray = JSONArray();
        val jsonDeviceInfo = JSONObject()

        jsonDeviceInfo.put("manufacturer", android.os.Build.MANUFACTURER)
        jsonDeviceInfo.put("os", "Android " + android.os.Build.VERSION.RELEASE)
        jsonDeviceInfo.put("cpu", CpuInfo.asJson())
        jsonDeviceInfo.put("sensors", sensors.asJson())

        records.put("key", android.os.Build.MODEL)
        records.put("value", jsonDeviceInfo)

        recordsArray.put(records)

        jsonBody.put("records", recordsArray)

        NetworkTask.getInstance(applicationContext).sendRequest(jsonBody)

        emitDeviceEvent(applicationContext, "sensors", jsonBody.toString())

        handler.postDelayed(this, 15000)
      }
    }, 500)
  }

  private fun emitDeviceEvent(reactContext: ReactApplicationContext, eventName: String, eventData: String) =
      reactContext.getJSModule(DeviceEventManagerModule.RCTDeviceEventEmitter::class.java).emit(eventName, eventData)

  companion object {
    private val REACT_CLASS = "SensorService"
    private val FOREGROUND = "com.sensordata.SensorService"
  }
}