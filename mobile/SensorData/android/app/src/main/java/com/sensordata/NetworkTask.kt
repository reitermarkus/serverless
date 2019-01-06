package com.sensordata;

import com.facebook.react.bridge.NativeModule;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;
import com.facebook.react.bridge.Callback;

import com.android.volley.Request;
import com.android.volley.RequestQueue;
import com.android.volley.Response;
import com.android.volley.Response.ErrorListener;
import com.android.volley.Response.Listener;
import com.android.volley.toolbox.Volley;
import com.android.volley.NetworkResponse
import com.android.volley.toolbox.StringRequest;
import com.android.volley.toolbox.HttpHeaderParser;
import com.android.volley.VolleyError

import android.util.Log

import org.json.JSONObject

class NetworkTask(reactContext: ReactApplicationContext) : ReactContextBaseJavaModule(reactContext)  {
  companion object {
    private var queue : RequestQueue? = null
  }

  init {
    queue = Volley.newRequestQueue(this.getReactApplicationContext())
  }

  override fun getName() = "NetworkTask"

  @ReactMethod
  fun sendRequest(callback: Callback) {
    val url = "http://10.0.0.5:4000/sensor"

    val stringRequest = object : StringRequest(Request.Method.POST, url,
      Response.Listener<String> { response ->
        callback.invoke(response.toString())
      },
      Response.ErrorListener {
        fun onErrorResponse(error: VolleyError) {
          val errorRes = error.networkResponse
          Log.e("Error", String(errorRes.data, Charsets.UTF_8))
          callback.invoke(String(errorRes.data, Charsets.UTF_8))
        }
      }
    ) {
      override fun getBodyContentType() = "application/json"

      override fun getBody(): ByteArray {
        val jsonBody = JSONObject()
        jsonBody.put("username", "Shozib@gmail.com");
        jsonBody.put("password", "Shozib123")

        return jsonBody.toString().toByteArray()
      }
    }

    queue!!.add(stringRequest)
  }
}