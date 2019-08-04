import 'dart:async';

import 'dart:io';
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:sensor_data/list.dart';

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
    Map<String, dynamic> _cpuInfoDecode = <String, dynamic>{};

    void setCpuInfo(Map<String, dynamic> cpuDecode) {
      _cpuInfoDecode.clear();
      _cpuInfoDecode.putIfAbsent('cores', () => cpuDecode['cores']);
      cpuDecode['frequency'].forEach((k, v) => _cpuInfoDecode.putIfAbsent(k, () => v));

      if (mounted) {
        setState(() {
          _cpuInfo = _cpuInfoDecode;
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
    return SingleChildScrollView(
      child: Column(
        children: buildList(context, _cpuInfo)
      )
    );
  }
}
