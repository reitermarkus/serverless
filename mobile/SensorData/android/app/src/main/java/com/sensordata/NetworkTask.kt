package com.sensordata;

import com.android.volley.Request;
import com.android.volley.RequestQueue;
import com.android.volley.Response;
import com.android.volley.Response.ErrorListener;
import com.android.volley.Response.Listener;
import com.android.volley.toolbox.Volley;
import com.android.volley.NetworkResponse
import com.android.volley.toolbox.StringRequest;
import com.android.volley.VolleyError

import android.util.Log
import android.content.Context;

import org.json.JSONObject

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

  fun sendRequest(jsonBody: JSONObject) {
    val url = "http://10.0.0.5:4000/sensor"

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
      override fun getBodyContentType() = "application/json"
      override fun getBody(): ByteArray = jsonBody.toString().toByteArray()
    }

    queue!!.add(stringRequest)
  }
}