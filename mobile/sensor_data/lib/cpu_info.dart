import 'dart:async';

import 'dart:io';
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

class CpuInfo extends StatefulWidget {
  const CpuInfo({
    Key key
   }) : super(key: key);

  @override
  _CpuInfoState createState() => _CpuInfoState();
}

class _CpuInfoState extends State<CpuInfo> {
  Map<String, dynamic> _cpuInfo = <String, dynamic>{};
  static const _messageChannel = BasicMessageChannel<String>('sensor', StringCodec());
  static const _platform = const MethodChannel('sensor_data.flutter.dev/cpu_info');

  @override
  void initState() {
    super.initState();
    initPlatformState();
  }

  Future<void> initPlatformState() async {
    Map<String, dynamic> _cpuInfo = <String, dynamic>{};

    void setCpuInfo(Map<String, dynamic> cpuDecode) {
      _cpuInfo.clear();
      _cpuInfo.putIfAbsent('cores', () => cpuDecode['cores']);
      cpuDecode['frequency'].forEach((k, v) => _cpuInfo.putIfAbsent(k, () => v));

      if (mounted) {
        setState(() {
          _cpuInfo = _cpuInfo;
        });
      }
    }

    if (Platform.isAndroid) {
      setCpuInfo(jsonDecode(await _platform.invokeMethod('getCpuInfo')));

      _messageChannel.setMessageHandler((String sensorData) async {
        Map<String, dynamic> sensorDecode = jsonDecode(sensorData);
        sensorDecode = sensorDecode['records'][0]['value'];
        setCpuInfo(sensorDecode['cpu']);

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
    );
  }
}
