// Copyright 2017 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:async';

import 'dart:io';
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:device_info/device_info.dart';

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
  static final DeviceInfoPlugin deviceInfoPlugin = DeviceInfoPlugin();
  static const platform = const MethodChannel('sensor_data.flutter.dev/cpu_info');
  static const service = const MethodChannel('sensor_data.flutter.dev/service');
  static const _messageChannel = BasicMessageChannel<String>('sensor', StringCodec());
  Map<String, dynamic> _deviceData = <String, dynamic>{};
  Map<String, dynamic> _cpuInfo = <String, dynamic>{};
  Map<String, dynamic> _acceleration = <String, dynamic>{};

  @override
  void initState() {
    super.initState();
    initPlatformState();
  }

  Future<void> initPlatformState() async {
    Map<String, dynamic> deviceData;
    Map<String, dynamic> acceleration = <String, dynamic>{};
    Map<String, dynamic> cpuInfo = <String, dynamic>{};

    try {
      if (Platform.isAndroid) {
        deviceData = _readAndroidBuildData(await deviceInfoPlugin.androidInfo);

        //start service
        await service.invokeMethod('startService');

        _messageChannel.setMessageHandler((String sensorData) async {
          Map<String, dynamic> sensorDecode = jsonDecode(sensorData);
          sensorDecode = sensorDecode['records'][0]['value'];

          acceleration = sensorDecode['sensors']['acceleration'];

          cpuInfo.clear();
          Map<String, dynamic> cpuDecode = sensorDecode['cpu'];
          cpuInfo.putIfAbsent('cores', () => cpuDecode['cores']);
          cpuDecode['frequency'].forEach((k, v) => cpuInfo.putIfAbsent(k, () => v));

          setState(() {
            _cpuInfo = cpuInfo;
            _acceleration = acceleration;
          });

          return sensorData;
        });

      } else if (Platform.isIOS) {
        deviceData = _readIosDeviceInfo(await deviceInfoPlugin.iosInfo);
      }
    } on PlatformException {
      deviceData = <String, dynamic>{
        'Error:': 'Failed to get platform version.'
      };
    }

    if (!mounted) return;

    setState(() {
      _deviceData = deviceData;
      _cpuInfo = cpuInfo;
    });
  }

  Map<String, dynamic> _readAndroidBuildData(AndroidDeviceInfo build) {
    return <String, dynamic>{
      'brand': build.brand,
      'model': build.model,
      'device': build.device,
      'version.securityPatch': build.version.securityPatch,
      'version.sdkInt': build.version.sdkInt,
      'version.release': build.version.release,
      'version.previewSdkInt': build.version.previewSdkInt,
      'version.incremental': build.version.incremental,
      'version.codename': build.version.codename,
      'board': build.board,
      'bootloader': build.bootloader,
      'display': build.display,
      'hardware': build.hardware,
      'host': build.host,
      'id': build.id,
      'product': build.product,
      'supported32BitAbis': build.supported32BitAbis,
      'supported64BitAbis': build.supported64BitAbis,
      'tags': build.tags,
      'type': build.type,
      'isPhysicalDevice': build.isPhysicalDevice,
      'androidId': build.androidId,
    };
  }

  Map<String, dynamic> _readIosDeviceInfo(IosDeviceInfo data) {
    return <String, dynamic>{
      'name': data.name,
      'systemName': data.systemName,
      'systemVersion': data.systemVersion,
      'model': data.model,
      'localizedModel': data.localizedModel,
      'identifierForVendor': data.identifierForVendor,
      'isPhysicalDevice': data.isPhysicalDevice,
      'utsname.sysname:': data.utsname.sysname,
      'utsname.nodename:': data.utsname.nodename,
      'utsname.release:': data.utsname.release,
      'utsname.version:': data.utsname.version,
      'utsname.machine:': data.utsname.machine,
    };
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
        body: ListView(
          children: <Widget> [
            Container(
              child: ListTile(
                title: Text(
                  'Device Info',
                  style: Theme.of(context).textTheme.headline
                ),
              ),
            ),
            Container(
              color: Colors.white,
              child: ListView(
                shrinkWrap: true,
                physics: ClampingScrollPhysics(),
                children: _deviceData.keys.map((String property) {
                  return Column(
                    children: <Widget>[
                      Container(
                        padding: const EdgeInsets.all(12),
                        child: Row(
                          mainAxisSize: MainAxisSize.max,
                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                          children: <Widget>[
                            Container(
                              child: Text(
                                property,
                                style: TextStyle(
                                  fontWeight: FontWeight.bold,
                                  fontSize: 16
                                ),
                              ),
                            ),
                            Flexible(
                              child: Container(
                                child: Text(
                                  '${_deviceData[property]}',
                                  overflow: TextOverflow.ellipsis,
                                  style: TextStyle(
                                    fontSize: 16
                                  ),
                                ),
                              ),
                            ),
                          ],
                        ),
                      ),
                      FractionallySizedBox(
                        widthFactor: 0.95,
                        child: Container(color: Color.fromARGB(255, 220, 220, 220), height: 0.6),
                      )
                    ],
                  );
                }).toList(),
              ),
            ),

            Platform.isAndroid ? (
              ListView(
                shrinkWrap: true,
                children: <Widget>[
                  Container(
                    child: ListTile(
                      title: Text(
                        'CPU',
                        style: Theme.of(context).textTheme.headline
                      ),
                    ),
                  ),
                  Container(
                    color: Colors.white,
                    child: ListView(
                      shrinkWrap: true,
                      physics: ClampingScrollPhysics(),
                      children: _cpuInfo.keys.map((String property) {
                        return Column(
                          children: <Widget>[
                            Container(
                              padding: const EdgeInsets.all(12),
                              child: Row(
                                mainAxisSize: MainAxisSize.max,
                                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                                children: <Widget>[
                                  Container(
                                    child: Text(
                                      property,
                                      style: TextStyle(
                                        fontWeight: FontWeight.bold,
                                        fontSize: 16
                                      ),
                                    ),
                                  ),
                                  Flexible(
                                    child: Container(
                                      child: Text(
                                        '${_cpuInfo[property]}',
                                        overflow: TextOverflow.ellipsis,
                                        style: TextStyle(
                                          fontSize: 16
                                        ),
                                      ),
                                    ),
                                  ),
                                ],
                              ),
                            ),
                            FractionallySizedBox(
                              widthFactor: 0.95,
                              child: Container(color: Color.fromARGB(255, 220, 220, 220), height: 0.6),
                            )
                          ],
                        );
                      }).toList(),
                    )
                  )
                ],
              )
            ) : null,

            Platform.isAndroid ? (
              ListView(
                shrinkWrap: true,
                children: <Widget>[
                  Container(
                    child: ListTile(
                      title: Text(
                        'Acceleration',
                        style: Theme.of(context).textTheme.headline
                      ),
                    ),
                  ),
                  Container(
                    color: Colors.white,
                    child: ListView(
                      shrinkWrap: true,
                      physics: ClampingScrollPhysics(),
                      children: _acceleration.keys.map((String property) {
                        return Column(
                          children: <Widget>[
                            Container(
                              padding: const EdgeInsets.all(12),
                              child: Row(
                                mainAxisSize: MainAxisSize.max,
                                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                                children: <Widget>[
                                  Container(
                                    child: Text(
                                      property,
                                      style: TextStyle(
                                        fontWeight: FontWeight.bold,
                                        fontSize: 16
                                      ),
                                    ),
                                  ),
                                  Flexible(
                                    child: Container(
                                      child: Text(
                                        '${_acceleration[property]}',
                                        overflow: TextOverflow.ellipsis,
                                        style: TextStyle(
                                          fontSize: 16
                                        ),
                                      ),
                                    ),
                                  ),
                                ],
                              ),
                            ),
                            FractionallySizedBox(
                              widthFactor: 0.95,
                              child: Container(color: Color.fromARGB(255, 220, 220, 220), height: 0.6),
                            )
                          ],
                        );
                      }).toList(),
                    )
                  )
                ],
              )
            ) : null
          ]
        ),
      ),
    );
  }
}