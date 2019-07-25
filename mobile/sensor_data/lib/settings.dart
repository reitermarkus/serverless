import 'dart:async';

import 'package:flutter/services.dart';
import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';

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
        _interval = prefs.getInt('interval') ?? 15000;
        _url = prefs.getString('url') ?? 'http://10.0.0.198';
      });

      _intervalController.text = _interval.toString();
      _urlController.text = _url;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: <Widget>[
        Column(
          children: <Widget>[
            Container(
              margin: EdgeInsets.only(left: 20, right: 20, top: 15),
              child: TextField(
                controller: _intervalController,
                keyboardType: TextInputType.number,
                decoration: InputDecoration(
                  hintText: 'update interval',
                  icon: Icon(Icons.timer)
                ),
                style: new TextStyle(
                  fontSize: 18
                ),
                onChanged: (text) {
                  final parsedString = int.tryParse(text);

                  if (parsedString != null) {
                    if (mounted) {
                      setState(() {
                        _interval = int.parse(text);
                      });
                    }
                  }
                },
              )
            ),
            Container(
              margin: EdgeInsets.only(left: 20, right: 20, top: 20),
              child: TextField(
                controller: _urlController,
                keyboardType: TextInputType.url,
                decoration: InputDecoration(
                  hintText: 'url',
                  icon: Icon(Icons.dns)
                ),
                style: TextStyle(
                  fontSize: 18
                ),
                onChanged: (text) {
                  if (mounted) {
                    setState(() {
                      _url = text;
                    });
                  }
                },
              )
            ),
            Container(
              margin: EdgeInsets.only(top: 30),
              child: FractionallySizedBox(
                widthFactor: 0.9,
                child: ButtonTheme(
                  height: 45,
                  child: FlatButton(
                    child: Text(
                      'SAVE',
                      style: TextStyle(
                        fontSize: 16,
                        color: Colors.white
                      ),
                    ),
                    color: Colors.green,
                    onPressed: () async {
                      final prefs = await SharedPreferences.getInstance();

                      if (_interval >= 1000 && _interval <= 60000) {
                        prefs.setInt('interval', _interval);
                        prefs.setString('url', _url);
                        print(await _methodChannel.invokeMethod('changeSettings', {'interval' : _interval, 'url': _url}));

                        Scaffold.of(context).showSnackBar(
                          SnackBar(
                            content: Text('Saved settings sucessfully.')
                          )
                        );
                      } else {
                        Scaffold.of(context).showSnackBar(
                          SnackBar(
                            backgroundColor: Colors.red,
                            content: Text('The value "$_interval" for interval is not allowed. It must be in milliseconds and between 1000ms and 60000ms.')
                          )
                        );
                      }
                    },
                  ),
                )
              )
            )
          ],
        )
      ],
    );
  }
}
