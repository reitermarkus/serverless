import 'dart:async';

import 'dart:io';
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
  Map<String, dynamic> _gyroscope = <String, dynamic>{};
  Map<String, dynamic> _orientation = <String, dynamic>{};
  String _pressure = '';
  Timer _timer;

  static const _messageChannel = BasicMessageChannel<String>('sensor', StringCodec());
  static const _sensorChannel = const MethodChannel('sensor_data.flutter.dev/sensor');

  @override
  void initState() {
    super.initState();

    _timer = Timer.periodic(Duration(milliseconds: 500), (timer) {
      updateState();
    });

    initPlatformState();
  }

  @override
  void dispose() {
    _timer.cancel();
    super.dispose();
  }

  void setSensorInfo(Map<String, dynamic> sensorDecode) {
    if (mounted) {
      setState(() {
        _acceleration = sensorDecode['acceleration'];
        _gravity = sensorDecode['gravity'] ?? {};
        _magneticField = sensorDecode['magnetic_field'] ?? {};
        _gyroscope = sensorDecode['gyroscope'] ?? {};
        _orientation = sensorDecode['orientation'] ?? {};
        _pressure = sensorDecode['air_pressure'];
      });
    }
  }

  Future<void> updateState() async {
    if (Platform.isIOS) {
      double pressure = (await _sensorChannel.invokeMethod('getPressure'));

      List<double> acceleration = (await _sensorChannel.invokeMethod('getAcceleration')).cast<double>();
      List<double> gravity = (await _sensorChannel.invokeMethod('getGravity')).cast<double>();
      List<double> magneticField = (await _sensorChannel.invokeMethod('getMagneticField')).cast<double>();
      List<double> rotationRate = (await _sensorChannel.invokeMethod('getRotationRate')).cast<double>();
      List<double> attitude = (await _sensorChannel.invokeMethod('getAttitude')).cast<double>();

      setSensorInfo({
        'acceleration': {
          'x': acceleration[0],
          'y': acceleration[1],
          'z': acceleration[2],
        },
        'gravity': {
          'x': gravity[0],
          'y': gravity[1],
          'z': gravity[2],
        },
        'magnetic_field': {
          'x': magneticField[0],
          'y': magneticField[1],
          'z': magneticField[2],
        },
        'gyroscope': {
          'x': rotationRate[0],
          'y': rotationRate[1],
          'z': rotationRate[2],
        },
        'orientation': {
          'roll': attitude[0],
          'pitch': attitude[1],
          'yaw': attitude[2],
        },
        'air_pressure': "$pressure hPa",
      });
    }
  }

  Future<void> initPlatformState() async {
    await updateState();

    if (Platform.isAndroid) {
      setSensorInfo(jsonDecode(await _sensorChannel.invokeMethod('getSensorInfo')));

      _messageChannel.setMessageHandler((String sensorData) async {
        Map<String, dynamic> sensorDecode = jsonDecode(sensorData);
        sensorDecode = sensorDecode['records'][0]['value'];
        setSensorInfo(sensorDecode['sensors']);
        return sensorData;
      });
    }

    if (!mounted) return;
  }

  @override
  Widget build(BuildContext context) {
    return ListView(
      shrinkWrap: true,
      children: <Widget>[
        buildList(context, _acceleration, 'Acceleration'),
        buildList(context, _gravity, 'Gravity'),
        buildList(context, _magneticField, 'Magnetic Field'),
        buildList(context, _gyroscope, 'Gyroscope'),
        buildList(context, _orientation, 'Orientation'),
        Column(
          children: <Widget>[
            Container(
              child: ListTile(
                title: Text(
                  'Air pressure',
                  style: Theme.of(context).textTheme.headline
                ),
              ),
            ),
            Container(
              color: Colors.white,
              child: Column(
                children: <Widget>[
                  Container(
                    padding: const EdgeInsets.all(12),
                    child: Row(
                      mainAxisSize: MainAxisSize.max,
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: <Widget>[
                        Container(
                          child: Text(
                            'pressure',
                            style: TextStyle(
                              fontWeight: FontWeight.bold,
                              fontSize: 16
                            ),
                          ),
                        ),
                        Flexible(
                          child: Container(
                            child: Text(
                              '$_pressure',
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
              ),
            ),
          ]
        )
      ],
    );
  }
}
