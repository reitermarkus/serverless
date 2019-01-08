package com.sensordata;

import com.facebook.react.bridge.NativeModule;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;
import com.facebook.react.bridge.Callback;
import com.facebook.react.bridge.WritableNativeMap;

import java.io.*
import java.util.*

class StreamManager(reactContext: ReactApplicationContext) : ReactContextBaseJavaModule(reactContext) {
  companion object {
    public fun getCores() = Runtime.getRuntime().availableProcessors()

    public fun getCurrentFrequency(coreNumber: Int): String {
      var frequency = "Stopped"
      val filePath = "/sys/devices/system/cpu/cpu$coreNumber/cpufreq/scaling_cur_freq"

      try {
        val reader = RandomAccessFile(filePath, "r")
        val value = reader.readLine().toLong() / 1000
        reader.close()
        frequency = "${value}MHz"
      } catch (ignored: Exception) {
      }

      return frequency
    }
  }

  override fun getName() = "CpuInfo"

  @ReactMethod
  fun getCpuCores(callback: Callback) = callback.invoke(getCores())

  @ReactMethod
  fun getCoresInfo(callback: Callback) {
    var coresMap = WritableNativeMap()
    val numbersOfCores = getCores()

    for (i in 0 until numbersOfCores) {
      coresMap.putString("Core $i", getCurrentFrequency(i))
    }

    callback.invoke(coresMap)
  }
}
