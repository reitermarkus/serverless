// Copyright 2017 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:async';

import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import 'device_meta.dart';
import 'cpu_info.dart';
import 'acceleration.dart';

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
    new Acceleration()
  ];

  @override
  void initState() {
    super.initState();
    initPlatformState();
  }

  Future<void> initPlatformState() async {
    if (Platform.isAndroid) {
      //start service
      await service.invokeMethod('startService');
    }

    if (!mounted) return;
  }

  void _onItemTapped(int index) {
    setState(() {
      _selectedIndex = index;
    });
  }
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        backgroundColor: Color.fromARGB(255, 232, 232, 232),
        appBar: AppBar(
          backgroundColor: Color.fromARGB(255, 173, 34, 17),
          title: Text(
            Platform.isAndroid ? 'Android Device Info' : 'iOS Device Info'),
        ),
        body: _widgetOptions.elementAt(_selectedIndex),
        bottomNavigationBar: BottomNavigationBar(
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
          ],
          selectedItemColor: Color.fromARGB(255, 173, 34, 17),
          currentIndex: _selectedIndex,
          onTap: _onItemTapped,
        ),
      ),
    );
  }
}