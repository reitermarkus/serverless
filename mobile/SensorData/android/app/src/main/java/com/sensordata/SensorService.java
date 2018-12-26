package com.sensordata;

import com.sensordata.R;

import android.app.Activity;
import android.app.Service;
import android.app.NotificationChannel;
import android.app.Notification;
import android.app.NotificationManager;
import android.annotation.TargetApi;
import android.util.Log;
import android.os.IBinder;
import android.os.Build;
import android.content.Intent;
import android.app.PendingIntent;
import android.support.annotation.Nullable;
import android.support.v4.app.NotificationCompat;
import android.support.v4.app.NotificationManagerCompat;
import android.graphics.BitmapFactory;

public class SensorService extends Service {
  private static final String REACT_CLASS = "SensorService";

  @Override
  @TargetApi(Build.VERSION_CODES.M)
  public void onCreate() {
    Log.d(REACT_CLASS, "onCreate");
    super.onCreate();
  }

  @Override
  public void onDestroy() {
    Log.d(REACT_CLASS, "onDestroy");
    super.onDestroy();
  }

  @Override
  public int onStartCommand(Intent intent, int flags, int startId) {
    Log.d(REACT_CLASS, "onStartCommand, calling startForeground");
    createAndShowForegroundNotification(3313);
    return START_STICKY;
  }

  @Nullable
  @Override
  public IBinder onBind(Intent intent) {
    return null;
  }

  public NotificationCompat.Builder getNotificationBuilder(String channelId, int importance) {
    NotificationCompat.Builder builder;

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
      prepareChannel(channelId, importance);
      builder = new NotificationCompat.Builder(this, channelId);
    } else {
      builder = new NotificationCompat.Builder(this);
    }

    return builder;
  }

  @TargetApi(26)
  private void prepareChannel(String id, int importance) {
    final String appName = "Sensor Data";
    String description = "getting sensor data...";
    final NotificationManager nm = (NotificationManager) getSystemService(Activity.NOTIFICATION_SERVICE);

    if(nm != null) {
      NotificationChannel nChannel = nm.getNotificationChannel(id);

      if (nChannel == null) {
        nChannel = new NotificationChannel(id, appName, importance);
        nChannel.setDescription(description);
        nm.createNotificationChannel(nChannel);
      }
    }
  }

  private void createAndShowForegroundNotification(int notificationId) {

    final NotificationCompat.Builder builder = getNotificationBuilder(
          "com.sensorData.notification.CHANNEL_ID_FOREGROUND",
          NotificationManagerCompat.IMPORTANCE_LOW);

    String desc = "getting sensor data...";

    builder
      .setOngoing(true)
      .setSmallIcon(R.mipmap.ic_launcher_round)
      .setLargeIcon(BitmapFactory.decodeResource(getResources(), R.mipmap.ic_launcher))
      .setContentTitle("Sensor Data")
      .setContentText(desc)
      .setTicker(desc)
      .setWhen(System.currentTimeMillis());

    Notification notification = builder.build();

    startForeground(notificationId, notification);
  }
}