package com.sensor_data

import okhttp3.MediaType;
import okhttp3.OkHttpClient;
import okhttp3.Request;
import okhttp3.RequestBody;
import okhttp3.Response;
import okhttp3.Call;
import okhttp3.Callback;
import okhttp3.MediaType.Companion.toMediaType

import android.util.Log
import android.content.Context;

import org.json.JSONObject

import java.net.InetAddress
import java.io.IOException

class NetworkTask()  {
  companion object {
    private val client  = OkHttpClient()

    public fun sendRequest(jsonBody: JSONObject, topic: String, ip: String) {
      val url = "$ip:8082/topics/$topic"

      val contentType = "application/vnd.kafka.json.v2+json; charset=utf-8".toMediaType()
      val body = RequestBody.create(contentType, jsonBody.toString())

      val request = Request.Builder()
          .url(url)
          .post(body)
          .build()

      Log.d("NetworkTask SEND", "sending request to $url")

      client.newCall(request).enqueue(object: Callback {
        override fun onFailure(call: Call, e: IOException) {
          Log.e("NetworkTask ERROR", e.toString())
        }

        override fun onResponse(call: Call, response: Response) {
          response.use {
            if (!response.isSuccessful)
              Log.e("NetworkTask ERROR", response.toString())

            Log.d("NetworkTask RESPONSE", response.body!!.string())
          }
        }
      })
    }
  }
}
