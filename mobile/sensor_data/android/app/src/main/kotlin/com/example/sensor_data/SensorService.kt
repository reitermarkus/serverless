package com.sensor_data

import android.annotation.TargetApi
import android.app.Activity
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.Service
import android.content.Intent
import android.graphics.BitmapFactory
import android.os.Build
import android.os.IBinder
import android.support.v4.app.NotificationCompat
import android.support.v4.app.NotificationManagerCompat
import android.util.Log

class SensorService : Service() {
  @TargetApi(Build.VERSION_CODES.M)
  override fun onCreate() {
    Log.d(FLUTTER_CLASS, "onCreate")
    super.onCreate()
  }

  override fun onDestroy() {
    Log.d(FLUTTER_CLASS, "onDestroy")
    super.onDestroy()
    SensorServiceModule.getInstance().stopService = true
  }

  override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
    if (intent?.getAction().equals("stop_service")) {
      Log.d(FLUTTER_CLASS, "onStartCommand, stopping service")
      stopSelf()
    } else {
      Log.d(FLUTTER_CLASS, "onStartCommand, calling startForeground")
      createAndShowForegroundNotification(3313)
    }

    return START_STICKY
  }

  override fun onBind(intent: Intent?): IBinder? = null

  fun getNotificationBuilder(channelId: String, importance: Int): NotificationCompat.Builder {
    val builder: NotificationCompat.Builder

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
      prepareChannel(channelId, importance)
      builder = NotificationCompat.Builder(this, channelId)
    } else {
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

  private fun createAndShowForegroundNotification(notificationId: Int) {
    val builder = getNotificationBuilder(
                  "com.sensorData.notification.CHANNEL_ID_FOREGROUND",
                  NotificationManagerCompat.IMPORTANCE_LOW)

    val desc = "getting sensor data..."

    // https://stackoverflow.com/questions/30422452/how-to-stop-service-from-its-own-foreground-notification/35171958
    val intent = Intent(this, SensorService::class.java)
    intent.setAction("stop_service")
    val stopAction = PendingIntent.getService(this, 0, intent, PendingIntent.FLAG_CANCEL_CURRENT)

    builder
      .setOngoing(true)
      .setSmallIcon(R.mipmap.ic_launcher)
      .setLargeIcon(BitmapFactory.decodeResource(getResources(), R.mipmap.ic_launcher))
      .addAction(R.mipmap.ic_launcher, "Stop", stopAction)
      .setContentTitle("Sensor Data")
      .setContentText(desc)
      .setTicker(desc)
      .setWhen(System.currentTimeMillis())

    val notification = builder.build()
    startForeground(notificationId, notification)
  }

  companion object {
    private val FLUTTER_CLASS = "sensor_data_log SensorService"
  }
}
