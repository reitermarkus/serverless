package com.sensor_data

import com.android.volley.Request
import com.android.volley.RequestQueue
import com.android.volley.Response
import com.android.volley.Response.ErrorListener
import com.android.volley.Response.Listener
import com.android.volley.toolbox.Volley
import com.android.volley.NetworkResponse
import com.android.volley.toolbox.StringRequest
import com.android.volley.VolleyError
import com.android.volley.DefaultRetryPolicy

import android.util.Log
import android.content.Context;

import org.json.JSONObject

import java.net.InetAddress

class NetworkTask()  {
  companion object {
    private var queue : RequestQueue? = null

    @Volatile
    private var INSTANCE: NetworkTask? = null
    fun getInstance(context: Context) = INSTANCE ?: synchronized(this) {
      queue = queue ?: Volley.newRequestQueue(context)

      INSTANCE ?: NetworkTask().also {
        INSTANCE = it
      }
    }
  }

  fun sendRequest(jsonBody: JSONObject, topic: String, ip: String) {
    val url = "$ip:8082/topics/$topic"

    Log.d("NetworkTask", "sending request to $url.")

    val stringRequest = object : StringRequest(Request.Method.POST, url,
      Response.Listener<String> { response ->
        Log.d("NetworkTask", response.toString())
      },
      Response.ErrorListener {
        fun onErrorResponse(error: VolleyError) {
          val errorRes = error.networkResponse
          Log.e("Error", String(errorRes.data, Charsets.UTF_8))
        }
      }
    ) {
      override fun getBodyContentType() = "application/vnd.kafka.json.v2+json"
      override fun getBody(): ByteArray = jsonBody.toString().toByteArray()

      override fun parseNetworkResponse(response: NetworkResponse): Response<String> {
        Log.d("NetworkTaskStatusCode", response.statusCode.toString())
        return super.parseNetworkResponse(response)
      }
    }

    stringRequest.setRetryPolicy(
      DefaultRetryPolicy(
        1000,
        2,
        DefaultRetryPolicy.DEFAULT_BACKOFF_MULT
      )
    )

    queue!!.add(stringRequest)
  }
}
