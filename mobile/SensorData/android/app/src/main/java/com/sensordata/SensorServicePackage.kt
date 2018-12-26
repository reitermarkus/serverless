package com.sensordata;

import com.facebook.react.ReactPackage;
import com.facebook.react.bridge.NativeModule;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.uimanager.ViewManager;
import java.util.*;

class SensorServicePackage : ReactPackage {
  override fun createNativeModules(reactContext: ReactApplicationContext) = listOf(SensorServiceModule(reactContext))
  override fun createViewManagers(reactContext: ReactApplicationContext) = Collections.emptyList<ViewManager<*, *>>()
}