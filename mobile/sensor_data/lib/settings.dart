import 'dart:async';
import 'dart:io';

import 'package:flutter/services.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:flutter_platform_widgets/flutter_platform_widgets.dart';

class Settings extends StatefulWidget {
  const Settings({
    Key key
   }) : super(key: key);

  @override
  _SettingsState createState() => _SettingsState();
}

class _SettingsState extends State<Settings> {
  int _interval = 0;
  String _url = '';
  final _intervalController = TextEditingController();
  final _urlController = TextEditingController();
  static const _methodChannel = const MethodChannel('sensor_data.flutter.dev/settings');

  @override
  void initState() {
    super.initState();
    initPlatformState();
  }

  Future<void> initPlatformState() async {
    final prefs = await SharedPreferences.getInstance();

    if (mounted) {
      setState(() {
        _interval = prefs.getInt('interval');
        _url = prefs.getString('url');
      });

      _intervalController.text = _interval.toString();
      _urlController.text = _url;
    }
  }

  void handleIntervalChange(String text) {
    final parsedString = int.tryParse(text);

    if (parsedString != null) {
      if (mounted) {
        setState(() {
          _interval = int.parse(text);
        });
      }
    }
  }

  void handleUrlChange(String text) {
    if (mounted) {
      setState(() {
        _url = text;
      });
    }
  }

  Future<void> onSave(BuildContext context) async {
    final prefs = await SharedPreferences.getInstance();

    if (_interval >= 1000 && _interval <= 60000) {
      prefs.setInt('interval', _interval);
      prefs.setString('url', _url);

      if (Platform.isAndroid) {
        print(await _methodChannel.invokeMethod('changeSettings', {'interval' : _interval, 'url': _url}));

        Scaffold.of(context).showSnackBar(
          SnackBar(
            content: Text('Saved settings sucessfully.')
          )
        );
      }

      // Dismiss keyboard.
      FocusScope.of(context).unfocus();
    } else {
      if (Platform.isAndroid) {
        Scaffold.of(context).showSnackBar(
          SnackBar(
            backgroundColor: Colors.red,
            content: Text('The value "$_interval" for interval is not allowed. It must be in milliseconds and between 1000ms and 60000ms.')
          )
        );
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final buttonColor = Color.fromARGB(255, 0, 200, 120);

    final cupertinoTextFieldDecoration = BoxDecoration(
      color: Colors.white,
      border: Border.all(
        color: CupertinoColors.lightBackgroundGray,
        width: 1.0 / 3.0,
      ),
      borderRadius: BorderRadius.circular(8.0),
    );

    final textFieldHeight = Platform.isIOS ? 44.0 : null;
    final buttonHeight = Platform.isAndroid ? 45.0 : null;

    return SingleChildScrollView(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        mainAxisAlignment: MainAxisAlignment.start,
        children: <Widget>[
          Container(
            height: textFieldHeight,
            margin: EdgeInsets.only(left: 20, right: 20, top: 20),
            child: PlatformTextField(
              android: (_) => MaterialTextFieldData(
                decoration: InputDecoration(
                  hintText: 'update interval',
                  icon: Icon(Icons.timer)
                ),
                style: TextStyle(
                  fontSize: 18
                )
              ),
              ios: (_) => CupertinoTextFieldData(
                placeholder: 'Update Interval',
                decoration: cupertinoTextFieldDecoration,
              ),
              controller: _intervalController,
              keyboardType: TextInputType.number,
              onChanged: handleIntervalChange,
            )
          ),
          Container(
            height: textFieldHeight,
            margin: EdgeInsets.only(left: 20, right: 20, top: 20),
            child: PlatformTextField(
              android: (_) => MaterialTextFieldData(
                decoration: InputDecoration(
                  hintText: 'URL',
                  icon: Icon(Icons.dns)
                ),
                style: TextStyle(
                  fontSize: 18
                )
              ),
              ios: (_) => CupertinoTextFieldData(
                placeholder: 'URL',
                decoration: cupertinoTextFieldDecoration,
              ),
              controller: _urlController,
              keyboardType: TextInputType.url,
              onChanged: handleUrlChange,
            )
          ),
          Container(
            margin: EdgeInsets.only(left: 20, right: 20, top: 20),
            height: buttonHeight,
            child: FractionallySizedBox(
              widthFactor: 1.0,
              child: PlatformButton(
                androidFlat: (_) => MaterialFlatButtonData(color: buttonColor),
                child: PlatformText(
                  'Save',
                  style: TextStyle(fontSize: 16, color: Colors.white),
                ),
                color: buttonColor,
                onPressed: () async { await onSave(context); },
              )
            )
          )
        ]
      )
    );
  }
}
