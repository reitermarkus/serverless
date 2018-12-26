package com.sensordata;

import android.support.annotation.Nullable;

import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;
import com.facebook.react.bridge.WritableMap;
import com.facebook.react.modules.core.DeviceEventManagerModule;

import java.util.HashMap;
import java.util.Map;
import android.util.Log;
import com.facebook.react.bridge.Promise;
import android.content.Intent;
import android.app.PendingIntent;
import android.graphics.BitmapFactory;

import com.sensordata.ForegroundService;

public class ForegroundServiceModule extends ReactContextBaseJavaModule {
    public static final String REACT_CLASS = "ForegroundService";
    private static ReactApplicationContext reactContext = null;

    public ForegroundServiceModule(ReactApplicationContext context) {
      super(context);
      reactContext = context;
    }

    @Override
    public String getName() {
      return REACT_CLASS;
    }

    @ReactMethod
    public void startService(Promise promise) {
      Log.d(REACT_CLASS, "startService");
      try {
        Intent intent = new Intent(ForegroundService.FOREGROUND);
        intent.setClass(this.getReactApplicationContext(), ForegroundService.class);
        getReactApplicationContext().startService(intent);
        Log.d(REACT_CLASS, "startService, success");
        promise.resolve(true);
      } catch (Exception e) {
        Log.d(REACT_CLASS, "startService failed!");
        promise.reject(e);
      }
    }

    @ReactMethod
    public void stopService(Promise promise) {
      Log.d(REACT_CLASS, "stopService");
      try {
        Intent intent = new Intent(ForegroundService.FOREGROUND);
        intent.setClass(this.getReactApplicationContext(), ForegroundService.class);
        this.getReactApplicationContext().stopService(intent);
      } catch (Exception e) {
        Log.d(REACT_CLASS, "stopService failed!");
        promise.reject(e);
      }

      promise.resolve(true);
    }

    private static void emitDeviceEvent(String eventName, @Nullable WritableMap eventData) {
      reactContext.getJSModule(DeviceEventManagerModule.RCTDeviceEventEmitter.class).emit(eventName, eventData);
    }
}