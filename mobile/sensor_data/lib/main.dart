// Copyright 2017 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:async';

import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/services.dart';
import 'package:flutter_platform_widgets/flutter_platform_widgets.dart';

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
  List<Widget> _widgets;
  List<BottomNavigationBarItem> _items;

  @override
  void initState() {
    super.initState();

    _widgets = <Widget>[
      new DeviceMeta(),
      new Sensors(),
      new Settings()
    ];

    _items = [
      BottomNavigationBarItem(
        icon: PlatformWidget(
          ios: (_) => Icon(IconData(0xf3a2, fontFamily: CupertinoIcons.iconFont, fontPackage: CupertinoIcons.iconFontPackage)),
          android: (_) => Icon(Icons.phone_android),
        ),
        title: Text('Device Info'),
      ),
      BottomNavigationBarItem(
        icon: PlatformWidget(
          ios: (_) => Icon(IconData(0xf493, fontFamily: CupertinoIcons.iconFont, fontPackage: CupertinoIcons.iconFontPackage)),
          android: (_) => Icon(Icons.settings_remote),
        ),
        title: Text('Sensors'),
      ),
      BottomNavigationBarItem(
        icon: PlatformWidget(
          ios: (_) => Icon(CupertinoIcons.settings),
          android: (_) => Icon(Icons.settings),
        ),
        title: Text('Settings'),
      ),
    ];

    if (Platform.isAndroid) {
      _widgets.insert(1, new CpuInfo());

      _items.insert(1, BottomNavigationBarItem(
        icon: Icon(Icons.memory),
        title: Text('CPU'),
      ));
    }

    initPlatformState();
  }

  Future<void> initPlatformState() async {
    final prefs = await SharedPreferences.getInstance();

    // Initialize defaults.
    final url = prefs.getString('url');
    if (url == null) { prefs.setString('url', 'http://0.0.0.0'); }
    final interval = prefs.getInt('interval');
    if (interval == null) { prefs.setInt('interval', 15000); }

    if (Platform.isAndroid) {
      // Start service.
      await service.invokeMethod('startService', {
        'url' : prefs.getString('url'),
        'interval' : prefs.getInt('interval')
      });
    }
  }

  void _onItemTapped(int index) {
    setState(() {
      _selectedIndex = index;
    });
  }

  @override
  Widget build(BuildContext context) {

    final primaryColor = Platform.isIOS ? Color.fromARGB(255, 200, 50, 80) : Color.fromARGB(255, 173, 34, 17);

    final androidTheme = ThemeData(
      primaryColor: primaryColor,
      accentColor: Color.fromARGB(255, 210, 210, 210),
      backgroundColor: Colors.white,
    );

    final iosTheme = CupertinoThemeData(
      primaryColor: primaryColor,
      primaryContrastingColor: Colors.white,
      barBackgroundColor: primaryColor,
      scaffoldBackgroundColor: CupertinoColors.extraLightBackgroundGray,
      textTheme: CupertinoTextThemeData(
        navTitleTextStyle: CupertinoTextThemeData().navTitleTextStyle.apply(
          color: Colors.white,
        ),
      ),
    );

    return PlatformApp(
      debugShowCheckedModeBanner:  Platform.isIOS ? false : true,
      android: (_) => new MaterialAppData(theme: androidTheme),
      ios: (_) => new CupertinoAppData(theme: iosTheme),
      home: PlatformScaffold(
        // iosContentPadding: true,
        android: (_) => MaterialScaffoldData(backgroundColor: Color.fromARGB(255, 232, 232, 232)),
        appBar: PlatformAppBar(
          ios: (_) => CupertinoNavigationBarData(backgroundColor: primaryColor),
          title: _items[_selectedIndex].title,
        ),
        body: _widgets[_selectedIndex],
        bottomNavBar: PlatformNavBar(
          android: (_) => MaterialNavBarData(selectedItemColor: androidTheme.primaryColor, unselectedItemColor: androidTheme.accentColor),
          ios: (_) => CupertinoTabBarData(backgroundColor: Colors.white),
          items: _items,
          currentIndex: _selectedIndex,
          itemChanged: _onItemTapped,
        ),
      ),
    );
  }
}
