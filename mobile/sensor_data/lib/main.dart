// Copyright 2017 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:async';

import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import 'device_meta.dart';
import 'cpu_info.dart';
import 'sensors.dart';
import 'settings.dart';
import 'package:shared_preferences/shared_preferences.dart';

void main() {
  runZoned(() {
    runApp(SensorData());
  }, onError: (dynamic error, dynamic stack) {
    print(error);
    print(stack);
  });
}

class SensorData extends StatefulWidget {
  @override
  _SensorDataState createState() => _SensorDataState();
}

class _SensorDataState extends State<SensorData> {
  static const service = const MethodChannel('sensor_data.flutter.dev/service');
  static int _selectedIndex = 0;

  static List<Widget> _widgetOptions = <Widget>[
    new DeviceMeta(),
    new CpuInfo(),
    new Sensors(),
    new Settings()
  ];

  @override
  void initState() {
    super.initState();
    initPlatformState();
  }

  Future<void> initPlatformState() async {
    final prefs = await SharedPreferences.getInstance();

    // Initialize defaults.
    final url = prefs.getString('url');
    if (url == null) { prefs.setString('url', 'http://10.0.0.198'); }
    final interval = prefs.getInt('interval');
    if (interval == null) { prefs.setInt('interval', 15000); }

    if (Platform.isAndroid) {
      // Start service.
      await service.invokeMethod('startService', {'url' : prefs.getString('url')});
    }
  }

  void _onItemTapped(int index) {
    setState(() {
      _selectedIndex = index;
    });
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData(primaryColor: Color.fromARGB(255, 173, 34, 17)),
      home: Scaffold(
        backgroundColor: Color.fromARGB(255, 232, 232, 232),
        appBar: AppBar(
          title: Text(
            Platform.isAndroid ? 'Android Device Info' : 'iOS Device Info'),
        ),
        body: _widgetOptions.elementAt(_selectedIndex),
        bottomNavigationBar: BottomNavigationBar(
          type: BottomNavigationBarType.fixed,
          items: const <BottomNavigationBarItem>[
            BottomNavigationBarItem(
              icon: Icon(Icons.phone_android),
              title: Text('Device Info'),
            ),
            BottomNavigationBarItem(
              icon: Icon(Icons.memory),
              title: Text('CPU'),
            ),
            BottomNavigationBarItem(
              icon: Icon(Icons.settings_remote),
              title: Text('Sensors'),
            ),
            BottomNavigationBarItem(
              icon: Icon(Icons.settings),
              title: Text('Settings'),
            ),
          ],
          selectedItemColor: Color.fromARGB(255, 173, 34, 17),
          currentIndex: _selectedIndex,
          onTap: _onItemTapped,
        ),
      ),
    );
  }
}
