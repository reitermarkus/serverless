import 'dart:async';

import 'dart:io';
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

class Acceleration extends StatefulWidget {
  const Acceleration({
    Key key
   }) : super(key: key);

  @override
  _AccelerationState createState() => _AccelerationState();
}

class _AccelerationState extends State<Acceleration> {
  Map<String, dynamic> _acceleration = <String, dynamic>{};
  static const _messageChannel = BasicMessageChannel<String>('sensor', StringCodec());
    static const _sensorChannel = const MethodChannel('sensor_data.flutter.dev/sensor');

  @override
  void initState() {
    super.initState();
    initPlatformState();
  }

  Future<void> initPlatformState() async {
    Map<String, dynamic> acceleration = <String, dynamic>{};

    void setSensorInfo(Map<String, dynamic> sensorDecode) {
      acceleration = sensorDecode['acceleration'];

      if (mounted) {
        setState(() {
          _acceleration = acceleration;
        });
      }
    }

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
    );
  }
}
