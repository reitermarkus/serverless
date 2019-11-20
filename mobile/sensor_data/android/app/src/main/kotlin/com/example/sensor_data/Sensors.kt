package com.sensor_data

import android.content.Context
import android.hardware.Sensor
import android.hardware.SensorEvent
import android.hardware.SensorEventListener
import android.hardware.SensorManager
import kotlin.math.PI
import org.json.JSONObject

class Sensors(private val manager: SensorManager) : SensorEventListener {
  companion object {
    private var _sensorData: JSONObject = JSONObject()

    @Volatile
    private var INSTANCE: Sensors? = null
    fun getInstance(context: Context) = INSTANCE ?: synchronized(this) {
      INSTANCE ?: Sensors(context.getSystemService(Context.SENSOR_SERVICE) as SensorManager).also {
        INSTANCE = it
      }
    }
  }

  init {
    for (sensor in manager.getSensorList(Sensor.TYPE_ALL)) {
      manager.registerListener(this, sensor, SensorManager.SENSOR_DELAY_NORMAL)
    }
  }

  public fun asJson(): JSONObject {
    return _sensorData
  }

  override fun onAccuracyChanged(sensor: Sensor, accuracy: Int) { }

  fun xyz(event: SensorEvent): JSONObject {
    val xyz = JSONObject()

    xyz.put("x", event.values[0])
    xyz.put("y", event.values[1])
    xyz.put("z", event.values[2])

    return xyz
  }

  override fun onSensorChanged(event: SensorEvent) {
    when (event.sensor.type) {
      Sensor.TYPE_ACCELEROMETER, Sensor.TYPE_LINEAR_ACCELERATION ->
        _sensorData.put("acceleration", xyz(event))
      Sensor.TYPE_GRAVITY ->
        _sensorData.put("gravity", xyz(event))
      Sensor.TYPE_GYROSCOPE ->
        _sensorData.put("rotation_rate", xyz(event))
      Sensor.TYPE_ROTATION_VECTOR ->
        _sensorData.put("rotation", xyz(event))
      Sensor.TYPE_MAGNETIC_FIELD ->
        _sensorData.put("magnetic_field", xyz(event))
      Sensor.TYPE_ORIENTATION -> {
        val yaw = (if (event.values[0] > 180.0) (event.values[0] - 360.0).toFloat() else event.values[0]) / 180.0 * PI
        val pitch = event.values[1] / 180.0 * PI
        val roll = event.values[2] / 180.0 * PI

        var orientation = JSONObject()

        orientation.put("yaw", yaw)
        orientation.put("pitch", pitch)
        orientation.put("roll", roll)

        _sensorData.put("orientation", orientation)
      }
      Sensor.TYPE_PROXIMITY ->
        _sensorData.put("proximity", event.values[0])
      Sensor.TYPE_AMBIENT_TEMPERATURE ->
        _sensorData.put("air_temperature", event.values[0])
      Sensor.TYPE_LIGHT ->
        _sensorData.put("illuminance", event.values[0])
      Sensor.TYPE_PRESSURE ->
        _sensorData.put("pressure", event.values[0])
      Sensor.TYPE_RELATIVE_HUMIDITY ->
        _sensorData.put("relative_humidity", event.values[0])
      Sensor.TYPE_GYROSCOPE_UNCALIBRATED ->
        _sensorData.put("rotation_rate_uncalibrated", xyz(event))
      Sensor.TYPE_MAGNETIC_FIELD_UNCALIBRATED ->
        _sensorData.put("magnetic_field_uncalibrated", xyz(event))
      Sensor.TYPE_GEOMAGNETIC_ROTATION_VECTOR ->
        _sensorData.put("geomagnetic_rotation", xyz(event))
    }
  }
}
