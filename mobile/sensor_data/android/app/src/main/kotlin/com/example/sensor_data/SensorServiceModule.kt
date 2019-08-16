package com.sensor_data

import java.util.HashMap
import java.text.SimpleDateFormat
import java.util.Date

import android.util.Log
import android.content.Intent
import android.app.PendingIntent
import android.graphics.BitmapFactory
import android.hardware.SensorManager
import android.os.Handler
import android.provider.Settings
import android.content.Context

import org.json.JSONObject
import org.json.JSONArray

class SensorServiceModule() {
  var url: String = "http://0.0.0.0"
    get() = field
    set(value) {
      field = value
    }

  var updateInterval: Int = 15000
    set(value) {
      field = value
    }

  @Throws(Exception::class)
  private fun resetService(context: Context) {
    val intent = Intent(FOREGROUND)
    intent.setClass(context, SensorService::class.java)
    context.stopService(intent)
  }

  fun startService(ip: String, interval: Int, context: Context) : Pair<Boolean, String> {
    url = ip
    updateInterval = interval

    try {
      //resetService(context)
      val intent = Intent(FOREGROUND)
      intent.setClass(context, SensorService::class.java)
      context.stopService(intent)
      context.startService(intent)
      networkLoop(context)
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

  private fun networkLoop(context: Context) {
    val handler = Handler()
    val sensors = Sensors.getInstance(context)

    val registerDevRecords = JSONObject()
    val registerDevRecordsArray = JSONArray()

    val registerDevOuter = JSONObject()
    val registerDev = JSONObject()

    val id = Settings.Secure.getString(context.getContentResolver(), Settings.Secure.ANDROID_ID)

    registerDev.put("id", id)
    registerDev.put("name", Settings.Secure.getString(context.getContentResolver(), "bluetooth_name"))

    registerDevOuter.put("value", registerDev)
    registerDevRecordsArray.put(registerDevOuter)
    registerDevRecords.put("records", registerDevRecordsArray)

    Log.d("REGISTER", registerDevRecords.toString())

    NetworkTask.getInstance(context).sendRequest(registerDevRecords, "register-device", url)

    handler.postDelayed(object : Runnable {
      override fun run() {
        val sensorsObj = sensors.asJson()

        val records = JSONObject()
        val recordsArray = JSONArray()

        val cpuOuter = JSONObject()
        val cpu = JSONObject()

        cpu.put("type", "CPU")
        cpu.put("value", CpuInfo.asJson())
        cpu.put("device_id", id)

        val date = SimpleDateFormat("yyyy-MM-dd HH:mm:ss")

        cpu.put("time", date.format(Date()))

        cpuOuter.put("value", cpu)
        recordsArray.put(cpuOuter)

        val keys = sensorsObj.names()

        for (i in 0 until keys.length()) {
          val recordOuter = JSONObject()
          val record = JSONObject()

          val type = keys.getString(i)

          record.put("type", type)
          record.put("value", sensorsObj[type])
          record.put("device_id", id)

          record.put("time", date.format(Date()))

          recordOuter.put("value", record)

          recordsArray.put(recordOuter)
        }

        records.put("records", recordsArray)

        Log.d("DATA", records.toString())

        NetworkTask.getInstance(context).sendRequest(records, "sensor", url)

        handler.postDelayed(this, updateInterval.toLong())
      }
    }, 1500)
  }

  companion object {
    private val FLUTTER_CLASS = "SensorService"
    private val FOREGROUND = "com.sensordata.SensorService"
  }
}