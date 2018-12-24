package com.sensordata;

import com.facebook.react.bridge.NativeModule;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;
import com.facebook.react.bridge.Callback;

public class CpuInfo extends ReactContextBaseJavaModule {
  public CpuInfo(ReactApplicationContext reactContext) {
    super(reactContext);
  }

  @Override
  public String getName() {
    return "CpuInfo";
  }

  @ReactMethod
  public void getCpuCores(Callback callback) {
    callback.invoke(Runtime.getRuntime().availableProcessors());
  }
}
