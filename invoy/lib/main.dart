import 'package:flutter/material.dart';
import 'package:invoy/src/rust/api/simple.dart';
import 'package:invoy/src/rust/frb_generated.dart';
import 'package:window_manager/window_manager.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
  windowManager.waitUntilReadyToShow().then((_) async {
    await windowManager.setTitleBarStyle(TitleBarStyle.hidden);
  });
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: Text(
            'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`',
          ),
        ),
      ),
    );
  }
}
