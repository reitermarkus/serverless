package com.sensor_data
import java.io.*
import java.util.*

import org.json.JSONObject

class CpuInfo() {
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

    public fun asJson(): JSONObject {
      val cpu = JSONObject()
      val frequency = JSONObject()
      val numbersOfCores = CpuInfo.getCores()

      for (i in 0 until numbersOfCores) {
        frequency.put("Core $i", CpuInfo.getCurrentFrequency(i))
      }

      cpu.put("cores", numbersOfCores)
      cpu.put("frequency", frequency)

      return cpu
    }
  }
}
