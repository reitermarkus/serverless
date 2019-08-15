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
  static const _platform = const MethodChannel('sensor_data.flutter.dev/cpu_info');
  Timer _timer;

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

  void setCpuInfo(Map<String, dynamic> cpuInfo) {
    if (mounted && cpuInfo.isNotEmpty) {
      Map<String, dynamic> _cpuInfoDecode = <String, dynamic>{};
      _cpuInfoDecode.putIfAbsent('cores', () => cpuInfo['cores']);
      cpuInfo['frequency'].forEach((k, v) => _cpuInfoDecode.putIfAbsent(k, () => v));

      setState(() {
        _cpuInfo = _cpuInfoDecode;
      });
    }
  }

  Future<void> updateState() async {
    setCpuInfo(jsonDecode(await _platform.invokeMethod('getCpuInfo')));
  }

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      child: Container(
        padding: Platform.isAndroid ? EdgeInsets.only(top: 25) : null,
        child: Column(
          children: buildList(context, _cpuInfo)
        ),
      )
    );
  }
}
