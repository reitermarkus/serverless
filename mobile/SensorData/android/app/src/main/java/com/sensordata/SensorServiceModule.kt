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

  companion object {
    private val REACT_CLASS = "SensorService"
    private val FOREGROUND = "com.sensordata.SensorService"

    private fun emitDeviceEvent(reactContext: ReactApplicationContext, eventName: String, eventData: WritableMap?) =
      reactContext.getJSModule(DeviceEventManagerModule.RCTDeviceEventEmitter::class.java).emit(eventName, eventData)
  }
}