import 'dart:async';
import 'dart:io';

import 'package:flutter/services.dart';
import 'package:flutter/material.dart';

import 'package:shared_preferences/shared_preferences.dart';
import 'package:webview_flutter/webview_flutter.dart';

class SensorWebView extends StatefulWidget {
  const SensorWebView({
    Key key
   }) : super(key: key);

  @override
  _SensorWebViewState createState() => _SensorWebViewState();
}

class _SensorWebViewState extends State<SensorWebView> {
  String _url = '';

  @override
  void initState() {
    super.initState();
    initPlatformState();
  }

  Future<void> initPlatformState() async {
    final prefs = await SharedPreferences.getInstance();

    if (mounted) {
      setState(() {
        _url = prefs.getString('url');
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return _url != '' && _url != null ? WebView(
      initialUrl: '$_url:8080/function/ui/?headless=true',
      javascriptMode: JavascriptMode.unrestricted,
      onPageStarted: (String url) {
        print('Page started loading: $url');
      },
      onPageFinished: (String url) {
        print('Page finished loading: $url');
      },
    ) : Text('You need to set a url in settings first.');
  }
}

