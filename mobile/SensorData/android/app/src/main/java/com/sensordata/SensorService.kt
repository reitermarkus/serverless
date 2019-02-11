package com.sensordata

import com.sensordata.R
import android.app.Activity
import android.app.Service
import android.app.NotificationChannel
import android.app.Notification
import android.app.NotificationManager
import android.annotation.TargetApi
import android.util.Log
import android.os.IBinder
import android.os.Build
import android.os.Handler
import android.content.Intent
import android.app.PendingIntent
import android.support.annotation.Nullable
import android.support.v4.app.NotificationCompat
import android.support.v4.app.NotificationManagerCompat
import android.graphics.BitmapFactory
import android.content.Context;
import android.hardware.SensorManager

import org.json.JSONObject
import org.json.JSONArray

import com.sensorData.Sensors

class SensorService:Service() {
  @TargetApi(Build.VERSION_CODES.M)
  override fun onCreate() {
    Log.d(REACT_CLASS, "onCreate")
    super.onCreate()
  }

  override fun onDestroy() {
    Log.d(REACT_CLASS, "onDestroy")
    super.onDestroy()
  }

  override fun onStartCommand(intent: Intent?, flags:Int, startId:Int): Int {
    Log.d(REACT_CLASS, "onStartCommand, calling startForeground")
    networkLoop()
    createAndShowForegroundNotification(3313)
    return START_STICKY
  }

  override fun onBind(intent: Intent?): IBinder? = null

  private fun networkLoop() {
    val handler = Handler()

    val manager = getApplicationContext().getSystemService(Context.SENSOR_SERVICE) as SensorManager
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

        Log.e("body", jsonBody.toString())
        NetworkTask.getInstance(getApplicationContext()).sendRequest(jsonBody)

        handler.postDelayed(this, 15000)
      }
    }, 1000)
  }

  fun getNotificationBuilder(channelId:String, importance:Int): NotificationCompat.Builder {
    val builder: NotificationCompat.Builder

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
      prepareChannel(channelId, importance)
      builder = NotificationCompat.Builder(this, channelId)
    }
    else {
      builder = NotificationCompat.Builder(this)
    }

    return builder
  }

  @TargetApi(26)
  private fun prepareChannel(id: String, importance: Int) {
    val appName = "Sensor Data"
    val description = "getting sensor data..."
    var nm = getSystemService(Activity.NOTIFICATION_SERVICE) as NotificationManager?
    var nChannel = nm?.getNotificationChannel(id)

    if (nChannel == null) {
      nChannel = NotificationChannel(id, appName, importance)
      nChannel.setDescription(description)
      nm?.createNotificationChannel(nChannel)
    }
  }

  private fun createAndShowForegroundNotification(notificationId:Int) {
    val builder = getNotificationBuilder(
                  "com.sensorData.notification.CHANNEL_ID_FOREGROUND",
                  NotificationManagerCompat.IMPORTANCE_LOW)

    val desc = "getting sensor data..."

    builder
      .setOngoing(true)
      .setSmallIcon(R.mipmap.ic_launcher_round)
      .setLargeIcon(BitmapFactory.decodeResource(getResources(), R.mipmap.ic_launcher))
      .setContentTitle("Sensor Data")
      .setContentText(desc)
      .setTicker(desc)
      .setWhen(System.currentTimeMillis())

    val notification = builder.build()
    startForeground(notificationId, notification)
  }

  companion object {
    private val REACT_CLASS = "SensorService"
  }
}