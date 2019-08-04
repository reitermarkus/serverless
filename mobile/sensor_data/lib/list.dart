import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter_platform_widgets/flutter_platform_widgets.dart';

Container border(double inset) {
  final borderColor = Platform.isIOS ? Color(0xFFBCBBC1) : Color.fromARGB(255, 220, 220, 220);
  final height = Platform.isIOS ? 1.0 / 3.0 : 0.6;
  final alignment = Platform.isIOS ? Alignment.centerRight : Alignment.center;
  final padding = Platform.isIOS ? EdgeInsets.only(left: inset) : EdgeInsets.symmetric(horizontal: inset / 2.0);

  return Container(
    color: Colors.white,
    alignment: alignment,
    height: height,
    child: Padding(
      padding: padding,
      child: Container(color: borderColor, height: height),
    ),
  );
}

List<Widget> buildList(BuildContext context, Map<String, dynamic> propertyMap, String title) {
  return <Widget>[
    PlatformWidget(
      android: (_) => Container(
        child: ListTile(
          title: Text(
            title,
            style: Theme.of(context).textTheme.headline,
          ),
        ),
      ),
      ios: (_) => Container(
        height: 55.0,
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16.0, vertical: 6.0),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.start,
            crossAxisAlignment: CrossAxisAlignment.end,
            children: <Widget>[
              Text(
                title.toUpperCase(),
                style: TextStyle(color: Color(0xFF6c6c71), fontSize: 13),
              ),
            ],
          ),
        )
      ),
    ),
  ] + propertyMap.keys.toList().asMap().map((index, String property) {
    final labelStyle = Platform.isIOS ?
      CupertinoTheme.of(context).textTheme.textStyle :
      TextStyle(fontWeight: FontWeight.bold, fontSize: 16);

    final valueStyle = Platform.isIOS ?
      CupertinoTheme.of(context).textTheme.textStyle.apply(color: CupertinoColors.inactiveGray) :
      TextStyle(fontSize: 16);

    final padding = Platform.isIOS ?
      const EdgeInsets.symmetric(horizontal: 16.0, vertical: 8.0) :
      const EdgeInsets.all(12);

    final height = Platform.isIOS ? 44.0 : null;

    final borderInset = Platform.isIOS ? 16.0 : 12.0;

    return MapEntry(index, Column(
      children: <Widget>[
        ...(index == 0 ? [border(0.0)] : []),
        Container(
          color: Colors.white,
          height: height,
          padding: padding,
          child: Row(
            mainAxisSize: MainAxisSize.max,
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: <Widget>[
              Container(
                child: Text(
                  property,
                  style: labelStyle,
                ),
              ),
              Flexible(
                child: Container(
                  child: Text(
                    '${propertyMap[property]}',
                    overflow: TextOverflow.ellipsis,
                    style: valueStyle,
                  ),
                ),
              ),
            ],
          ),
        ),
        index == propertyMap.length - 1 ? border(0.0) : border(borderInset),
      ],
    ));
  }).values.toList();
}
