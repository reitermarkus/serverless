import 'dart:async';

import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'list.dart';

class Sensors extends StatefulWidget {
  const Sensors({
    Key key
   }) : super(key: key);

  @override
  _SensorsState createState() => _SensorsState();
}

class _SensorsState extends State<Sensors> {
  Map<String, dynamic> _acceleration = <String, dynamic>{};
  Map<String, dynamic> _gravity = <String, dynamic>{};
  Map<String, dynamic> _magneticField = <String, dynamic>{};
  Map<String, dynamic> _rotationRate = <String, dynamic>{};
  Map<String, dynamic> _orientation = <String, dynamic>{};
  String _pressure = '';
  Timer _timer;

  static const _sensorChannel = const MethodChannel('sensor_data.flutter.dev/sensor');

  @override
  void initState() {
    super.initState();

    _timer = Timer.periodic(Duration(milliseconds: 500), (timer) async {
      await updateState();
    });

    updateState();
  }

  @override
  void dispose() {
    _timer.cancel();
    super.dispose();
  }

  void setSensorInfo(Map<String, dynamic> sensorInfo) {
    if (mounted && sensorInfo.isNotEmpty) {
      setState(() {
        _acceleration = {
          'x': "${sensorInfo['acceleration']['x']} m/s²",
          'y': "${sensorInfo['acceleration']['y']} m/s²",
          'z': "${sensorInfo['acceleration']['z']} m/s²",
        };
        _gravity = {
          'x': "${sensorInfo['gravity']['x']} m/s²",
          'y': "${sensorInfo['gravity']['y']} m/s²",
          'z': "${sensorInfo['gravity']['z']} m/s²",
        };
        _magneticField = {
          'x': "${sensorInfo['magnetic_field']['x']} µT",
          'y': "${sensorInfo['magnetic_field']['y']} µT",
          'z': "${sensorInfo['magnetic_field']['z']} µT",
        };
        _rotationRate = {
          'x': "${sensorInfo['rotation_rate']['x']} rad/s",
          'y': "${sensorInfo['rotation_rate']['y']} rad/s",
          'z': "${sensorInfo['rotation_rate']['z']} rad/s",
        };
        _orientation = {
          'yaw': "${sensorInfo['orientation']['yaw']} rad",
          'pitch': "${sensorInfo['orientation']['pitch']} rad",
          'roll': "${sensorInfo['orientation']['roll']} rad",
        };
        _pressure = "${sensorInfo['pressure']} hPa";
      });
    }
  }

  Future<void> updateState() async {
    setSensorInfo(jsonDecode(await _sensorChannel.invokeMethod('getSensorInfo')));
  }

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      child: Column(
        children: [
          buildTitle(context, 'Acceleration'),
          ...buildList(context, _acceleration),
          buildTitle(context, 'Gravity'),
          ...buildList(context, _gravity),
          buildTitle(context, 'Magnetic Field'),
          ...buildList(context, _magneticField),
          buildTitle(context, 'Rotation Rate'),
          ...buildList(context, _rotationRate),
          buildTitle(context, 'Orientation'),
          ...buildList(context, _orientation),
          buildTitle(context, 'Air Pressure'),
          ...buildList(context, {'pressure': _pressure}),
        ]
      )
    );
  }
}
