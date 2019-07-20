package com.example.sensor_data

import java.util.HashMap
import java.util.concurrent.atomic.AtomicLong;

import android.util.Log
import android.content.Intent
import android.app.PendingIntent
import android.graphics.BitmapFactory
import android.hardware.SensorManager
import android.os.Handler
import android.content.Context

import org.json.JSONObject
import org.json.JSONArray

class SensorServiceModule() {
  var url: String = "http://10.0.0.198"
    get() = field
    private set(value) {
      field = value
    }

  var updateInterval: Long = 15000
    private set(value) {
      field = value
    }

  @Throws(Exception::class)
  private fun resetService(context: Context) {
    val intent = Intent(FOREGROUND)
    intent.setClass(context, SensorService::class.java)
    context.stopService(intent)
  }

  fun startService(ip: String, context: Context, cb : (JSONObject) -> Unit) : Pair<Boolean, String> {
    url = ip

    try {
      //resetService(context)
      val intent = Intent(FOREGROUND)
      intent.setClass(context, SensorService::class.java)
      context.stopService(intent)
      context.startService(intent)
      networkLoop(context, cb)
      Log.d(FLUTTER_CLASS, "startService, successfull")

      return Pair(true, "")
    }
    catch (e: Exception) {
      Log.d(FLUTTER_CLASS, "startService failed!")
      return Pair (false, "startService failed!")
    }
  }

  fun stopService(context: Context) : Pair<Boolean, String>  {
    Log.d(FLUTTER_CLASS, "stopService")

    try {
      resetService(context)
    }
    catch (e: Exception) {
      Log.e(FLUTTER_CLASS, "stopService failed!")
      return Pair (false, "stopService failed!")
    }

    return Pair(true, "")
  }

  private fun networkLoop(context: Context, cb : (JSONObject) -> Unit) {
    val handler = Handler()
    val counter = AtomicLong(0L)

    handler.postDelayed(object : Runnable {
      override fun run() {
        val jsonBody = JSONObject()
        val records = JSONObject()
        val recordsArray = JSONArray();
        val jsonDeviceInfo = JSONObject()

        jsonDeviceInfo.put("manufacturer", android.os.Build.MANUFACTURER)
        jsonDeviceInfo.put("os", "Android " + android.os.Build.VERSION.RELEASE)
        jsonDeviceInfo.put("cpu", CpuInfo.asJson())
        jsonDeviceInfo.put("sensors", Sensors.getInstance(context).asJson())

        records.put("key", android.os.Build.MODEL)
        records.put("value", jsonDeviceInfo)

        recordsArray.put(records)

        jsonBody.put("records", recordsArray)

        cb(jsonBody)

        if (counter.addAndGet(1L) == (updateInterval / 500)) {
          NetworkTask.getInstance(context).sendRequest(jsonBody, url)
          counter.set(0L)
        }

        handler.postDelayed(this, 500)
      }
    }, 3000)
  }

  companion object {
    private val FLUTTER_CLASS = "SensorService"
    private val FOREGROUND = "com.sensordata.SensorService"
  }
}